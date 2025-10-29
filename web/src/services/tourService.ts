import {driver} from 'driver.js';
import 'driver.js/dist/driver.css';
import { removeMockDataAfterTour } from './mockTourData';
import Step1 from'../assets/step_1.png'
import Step2 from'../assets/step_2.png'
import Step3 from'../assets/step_3.png'
import Step4 from'../assets/step_4.png'
import Step5 from'../assets/step_5.png'
import Step6 from'../assets/step_6.png'
import Step7 from'../assets/step_7.png'
import Introduce from'../assets/sonar_introduce.png'
import Avatar from'../assets/sonar_avatar.jpg'

export interface TourStep {
  element?: string;
  popover: {
    title: string;
    description: string;
    side?: 'left' | 'right' | 'top' | 'bottom';
    align?: 'start' | 'center' | 'end';
  };
}

const tourGuideImages: Array<{ image: any; position: 'left' | 'right' }> = [
  { image: 'null', position: 'left' }, // Intro step
  { image: Step1, position: 'left' },
  { image: Step2, position: 'right' },
  { image: Step3, position: 'right' },
  { image: Step4, position: 'right' },
  { image: Step5, position: 'left' },
  { image: Step6, position: 'right' },
  { image: Step7, position: 'left' },
];

// Helper function to create tour guide image element
const createTourGuideImage = (imagePath: string, position: 'left' | 'right' = 'left'): HTMLElement => {
  const container = document.createElement('div');
  container.setAttribute('data-tour-guide-image', 'true');
  container.style.cssText = `
    position: fixed;
    ${position}: 20px;
    bottom: 120px;
    scale: 2;
    z-index: 10000;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.3s ease-in-out;
  `;
  
  const img = document.createElement('img');
  img.src = imagePath;
  img.alt = 'Sonar Tour Guide';
  img.style.cssText = `
    width: 250px;
    height: auto;
    max-height: 350px;
    object-fit: contain;
    filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));
  `;
  
  container.appendChild(img);
  return container;
};

// Clean up tour guide images
const cleanupTourGuideImages = () => {
  const tourGuideImages = document.querySelectorAll('[data-tour-guide-image]');
  tourGuideImages.forEach(img => {
    (img as HTMLElement).style.opacity = '0';
    setTimeout(() => img.remove(), 300);
  });
};

// Show tour guide image for specific step
const showTourGuideImage = (stepIndex: number) => {
  cleanupTourGuideImages();
  if (stepIndex >= 0 && stepIndex < tourGuideImages.length) {
    const config = tourGuideImages[stepIndex];
    // Skip showing tour guide image if image is 'null' or empty (for intro step)
    if (config.image && config.image !== 'null' && config.image.trim() !== '') {
      const imageElement = createTourGuideImage(config.image, config.position);
      document.body.appendChild(imageElement);
      // Fade in
      setTimeout(() => {
        imageElement.style.opacity = '1';
      }, 10);
    }
  }
};

export const tourSteps: TourStep[] = [
  {
    popover: {
      title: '👋 Hello! I\'m Sonar',
      description: `
        <div style="text-align: center; padding: 20px;">
          <div style="margin-bottom: 20px;">
            <img src="${Introduce}" alt="Sonar" style="width: 200px; height: auto; max-height: 300px; object-fit: contain; margin: 0 auto; display: block; border: none !important; box-shadow: none !important;" />
          </div>
          <p style="font-size: 18px; font-weight: 500; margin-bottom: 12px; color: #1f2937;">Hi there! I'm Sonar, your code quality tour guide! 👋</p>
          <p style="font-size: 14px; color: #6b7280; line-height: 1.6;">I'm here to help you explore our application and show you all the amazing features we have for managing your SonarQube projects. Let's get started on this journey together!</p>
        </div>
      `,
      side: 'bottom',
      align: 'center'
    }
  },
  {
    element: '[data-tour="add-project"]',
    popover: {
      title: '👋 Welcome to SonarQube Code Check Report!',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 60px; height: 60px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Hi! I'm Sonar, your code quality guide.</p>
            <p style="margin: 4px 0 0 0; font-size: 13px; color: #6b7280;">Let me show you around our application!</p>
          </div>
        </div>
        <p>First, you can create new projects by clicking this "Add New Project" button.</p>
      `,
      side: 'bottom',
      align: 'center'
    }
  },
  {
    element: '[data-tour="project-card"]',
    popover: {
      title: '📁 Project Management',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Here you can see all your projects!</p>
          </div>
        </div>
        <p>Each project card shows the project details, language, and creation date. Let's explore what you can do with each project!</p>
      `,
      side: 'right',
      align: 'start'
    }
  },
  {
    element: '[data-tour="generate-command"]',
    popover: {
      title: '⚡ Generate Sonar Command',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Ready to scan your code?</p>
          </div>
        </div>
        <p>Click this button to generate the SonarQube scan command for your project. This will create the exact command you need to run in your terminal!</p>
      `,
      side: 'top',
      align: 'center'
    }
  },
  {
    element: '[data-tour="get-results"]',
    popover: {
      title: '📊 Get Scan Results',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Let's see what we found!</p>
          </div>
        </div>
        <p>After running the scan command, click this button to fetch the results from SonarQube. This will get issues, coverage data, and quality gate status.</p>
        <div style="background: linear-gradient(135deg, #fef3c7, #fde68a); border: 2px solid #f59e0b; border-radius: 12px; padding: 16px; margin-top: 16px; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);">
          <div style="display: flex; align-items: center; gap: 8px;">
            <div style="font-size: 20px;">⚠️</div>
            <div>
              <p style="margin: 0; font-weight: 600; color: #92400e; font-size: 14px;">Action Required!</p>
              <p style="margin: 4px 0 0 0; font-weight: 500; color: #b45309; font-size: 13px;">Click the "Get Results" button below to continue the tour</p>
            </div>
          </div>
        </div>
      `,
      side: 'top',
      align: 'center'
    }
  },
  {
    element: '[data-tour="view-issues"]',
    popover: {
      title: '🔍 View Issues',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Let's dive into the details!</p>
          </div>
        </div>
        <p>Once you have results, you can click this button to open a detailed modal showing all the issues found in your code, organized by severity.</p>
      `,
      side: 'top',
      align: 'center'
    }
  },
  {
    element: '[data-tour="export-pdf"]',
    popover: {
      title: '📄 Export to PDF',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Share your results!</p>
          </div>
        </div>
        <p>Generate a comprehensive PDF report with all your project's issues, coverage metrics, and quality gate status. Perfect for sharing with your team!</p>
      `,
      side: 'top',
      align: 'center'
    }
  },
  {
    element: '[data-tour="quality-gate"]',
    popover: {
      title: '🚦 Quality Gate Status',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="${Avatar}" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
          <div>
            <p style="margin: 0; font-weight: 500;">Keep your code quality high!</p>
          </div>
        </div>
        <p>This shows your project's quality gate status. Green means it passes all quality checks, red means it needs attention. Let's keep your code healthy! 💪</p>
      `,
      side: 'left',
      align: 'center'
    }
  }
];

export const startTour = () => {
  let isWaitingForUserAction = false;
  let driverObj: any;
  
  const handleTourComplete = () => {
    removeMockDataAfterTour();
    localStorage.setItem('sonar-tour-completed', 'true');
    driverObj.destroy();
    window.location.reload();
  };
  
  driverObj = driver({
    showProgress: true,
    steps: tourSteps,
    popoverClass: 'driver-popover',
    allowClose: true,
    onCloseClick: () => {
      handleTourComplete();
      driverObj.destroy();
    },
    showButtons: ['next', 'previous', 'close'],
    onDestroyStarted: handleTourComplete,
    onDestroyed: handleTourComplete,
    onHighlighted: (_element, _step) => {
      const currentIndex = driverObj.getActiveIndex();

      // Show tour guide image for current step
      showTourGuideImage(currentIndex);

      // Step 4 is the "get-results" step (index 4 after intro step)
      isWaitingForUserAction = currentIndex === 4;

      if (currentIndex === 4) {
        const nextButton = document.querySelector('.driver-popover-next-btn');
        if (nextButton instanceof HTMLButtonElement) {
          nextButton.disabled = true;
          nextButton.style.opacity = '0.6';
        }
      } else {
        const nextButton = document.querySelector('.driver-popover-next-btn');
        if (nextButton) {
          (nextButton as HTMLElement).style.display = 'block';
        }
      }
    }
  });

  const handleGetResultsClick = (event: Event) => {
    const target = event.target as HTMLElement;
    
    const clickedElement = target.closest('[data-tour="get-results"]');

    if (clickedElement && isWaitingForUserAction) {
      isWaitingForUserAction = false;
      
      const nextButton = document.querySelector('.driver-popover-next-btn');
      if (nextButton instanceof HTMLButtonElement) {
        nextButton.disabled = true;
        nextButton.style.opacity = '0.6';
      }
      
      setTimeout(() => {
        driverObj.moveNext();
      }, 100);
    }
  };

  document.addEventListener('click', handleGetResultsClick);
  
  let lastDetectedIndex = -1;
  
  const checkLastStep = setInterval(() => {
    if (driverObj?.isActive()) {
      const currentIndex = driverObj.getActiveIndex();

      if (currentIndex !== lastDetectedIndex) {
        lastDetectedIndex = currentIndex;
      }
    }
  }, 100);

  const originalDestroy = driverObj.destroy.bind(driverObj);
  driverObj.destroy = () => {
    clearInterval(checkLastStep);
    document.removeEventListener('click', handleGetResultsClick);
    cleanupTourGuideImages();
    originalDestroy();
  };

  driverObj.drive();
  return driverObj;
};

export const hasCompletedTour = (): boolean => {
  return localStorage.getItem('sonar-tour-completed') === 'true';
};
