import {driver} from 'driver.js';
import 'driver.js/dist/driver.css';
import { removeMockDataAfterTour } from './mockTourData';

export interface TourStep {
  element: string;
  popover: {
    title: string;
    description: string;
    side?: 'left' | 'right' | 'top' | 'bottom';
    align?: 'start' | 'center' | 'end';
  };
}

export const tourSteps: TourStep[] = [
  {
    element: '[data-tour="add-project"]',
    popover: {
      title: '👋 Welcome to SonarQube Code Check Report!',
      description: `
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 60px; height: 60px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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
          <img src="/src/assets/sonar.jpg" alt="Sonar" style="width: 50px; height: 50px; border-radius: 50%; object-fit: cover;" />
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

      isWaitingForUserAction = currentIndex === 3;

      if (currentIndex === 3) {
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
    originalDestroy();
  };

  driverObj.drive();
  return driverObj;
};

export const hasCompletedTour = (): boolean => {
  return localStorage.getItem('sonar-tour-completed') === 'true';
};
