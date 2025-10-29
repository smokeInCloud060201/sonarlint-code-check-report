import jsPDF from 'jspdf';
import 'jspdf-autotable';
import type { ProjectResults } from '../types/api';

// Extend jsPDF type to include autoTable
declare module 'jspdf' {
  interface jsPDF {
    autoTable: (options: any) => jsPDF;
  }
}

interface Project {
  project_name: string;
  project_path: string;
  language: string;
}

export const exportIssuesToPDF = (project: Project, results: ProjectResults) => {
  const doc = new jsPDF();
  const currentDate = new Date().toLocaleString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });

  // Header with background and styling
  doc.setFillColor(66, 139, 202); // Blue background
  doc.rect(0, 0, 210, 50, 'F'); // Full width header background
  
  // Title with white text
  doc.setTextColor(255, 255, 255); // White text
  doc.setFontSize(24);
  doc.setFont('helvetica', 'bold');
  doc.text('SonarQube Issues Report', 20, 25);
  
  // Subtitle
  doc.setFontSize(16);
  doc.setFont('helvetica', 'normal');
  doc.text(project.project_name, 20, 35);
  
  // Reset text color for project details
  doc.setTextColor(0, 0, 0); // Black text
  doc.setFontSize(12);
  doc.setFont('helvetica', 'normal');
  
  // Project details in a styled box
  doc.setFillColor(248, 249, 250); // Light gray background
  doc.rect(15, 55, 180, 35, 'F');
  doc.setDrawColor(200, 200, 200);
  doc.rect(15, 55, 180, 35, 'S'); // Border
  
  doc.text(`Language: ${project.language.toUpperCase()}`, 20, 65);
  doc.text(`Scan Time: ${currentDate}`, 20, 75);
  doc.text(`Project Path: ${project.project_path}`, 20, 85);

  // Summary section with styled background - dynamic height based on severity count
  const severityCounts = results.issues?.issues.reduce((acc, issue) => {
    const severity = issue.severity.toLowerCase();
    acc[severity] = (acc[severity] || 0) + 1;
    return acc;
  }, {} as Record<string, number>) || {};

  const severityEntries = Object.entries(severityCounts);
  const rowsNeeded = Math.ceil(severityEntries.length / 4); // 4 items per row
  const summaryHeight = 50 + (rowsNeeded * 15); // Base height + additional rows

  doc.setFillColor(240, 248, 255); // Light blue background
  doc.rect(15, 95, 180, summaryHeight, 'F'); // Dynamic height based on content
  doc.setDrawColor(200, 200, 200);
  doc.rect(15, 95, 180, summaryHeight, 'S');
  
  doc.setFontSize(14);
  doc.setFont('helvetica', 'bold');
  doc.text('Summary', 20, 105);
  
  // Add Quality Gate Status to top-right of summary section
  if (results.quality_gate) {
    const qualityGateStatus = results.quality_gate.projectStatus.status;
    let statusColor: number[];
    let statusTextColor: number[];
    
    switch (qualityGateStatus) {
      case 'OK':
        statusColor = [34, 197, 94]; // Green
        statusTextColor = [255, 255, 255]; // White
        break;
      case 'WARN':
        statusColor = [251, 191, 36]; // Yellow
        statusTextColor = [0, 0, 0]; // Black
        break;
      case 'ERROR':
        statusColor = [239, 68, 68]; // Red
        statusTextColor = [255, 255, 255]; // White
        break;
      default:
        statusColor = [156, 163, 175]; // Gray
        statusTextColor = [255, 255, 255]; // White
    }
    
    // Draw quality gate status box in top-right
    doc.setFillColor(statusColor[0], statusColor[1], statusColor[2]);
    doc.rect(135, 95, 60, 12, 'F');
    doc.setTextColor(statusTextColor[0], statusTextColor[1], statusTextColor[2]);
    doc.setFontSize(10);
    doc.setFont('helvetica', 'bold');
    doc.text(`Quality Gate: ${qualityGateStatus}`, 150, 103);
    
    // Reset text color
    doc.setTextColor(0, 0, 0);
    doc.setFont('helvetica', 'normal');
  }
  
  doc.setFontSize(14);
  doc.setFont('helvetica', 'bold');
  doc.text(`Total Issues: ${results.issues?.paging.total || 0}`, 20, 115);

  // Add Coverage Information
  if (results.coverage && results.coverage.component && results.coverage.component.measures.length > 0) {
    doc.setFontSize(10);
    doc.setFont('helvetica', 'bold');
    doc.text('Coverage Metrics:', 20, 125);
    
    doc.setFontSize(9);
    doc.setFont('helvetica', 'normal');
    
    // Find main coverage metrics
    const coverageMeasure = results.coverage.component.measures.find(m => m.metric === 'coverage');
    const lineCoverageMeasure = results.coverage.component.measures.find(m => m.metric === 'line_coverage');
    const branchCoverageMeasure = results.coverage.component.measures.find(m => m.metric === 'branch_coverage');
    
    let yPos = 130;
    if (coverageMeasure) {
      doc.text(`Overall Coverage: ${coverageMeasure.value}%`, 25, yPos);
      yPos += 5;
    }
    if (lineCoverageMeasure) {
      doc.text(`Line Coverage: ${lineCoverageMeasure.value}%`, 25, yPos);
      yPos += 5;
    }
    if (branchCoverageMeasure) {
      doc.text(`Branch Coverage: ${branchCoverageMeasure.value}%`, 25, yPos);
      yPos += 5;
    }
  }


  // Count issues by severity with color coding - improved layout
  let xPos = 20;
  // Calculate yPos based on coverage section only (quality gate is now in top-right)
  let baseY = 125;
  if (results.coverage && results.coverage.component && results.coverage.component.measures.length > 0) {
    baseY += 25; // Add space for coverage section
  }
  let yPos = baseY; // Start position for severity indicators
  const maxItemsPerRow = 4; // Maximum items per row
  let itemsInCurrentRow = 0;

  // Sort severity by priority for consistent display
  const severityOrder = ['blocker', 'critical', 'major', 'minor', 'info'];
  const sortedSeverities = severityOrder.filter(severity => severityCounts[severity] > 0);

  sortedSeverities.forEach((severity) => {
    const count = severityCounts[severity];
    
    // Color coding for severity
    switch (severity.toLowerCase()) {
      case 'critical':
      case 'blocker':
        doc.setFillColor(220, 53, 69); // Red
        break;
      case 'major':
        doc.setFillColor(255, 193, 7); // Orange
        break;
      case 'minor':
        doc.setFillColor(255, 235, 59); // Yellow
        break;
      case 'info':
        doc.setFillColor(13, 202, 240); // Blue
        break;
      default:
        doc.setFillColor(108, 117, 125); // Gray
    }
    
    // Draw colored circle
    doc.circle(xPos + 5, yPos, 3, 'F');
    
    // Reset to black text
    doc.setTextColor(0, 0, 0);
    doc.text(`${severity.charAt(0).toUpperCase() + severity.slice(1)}: ${count}`, xPos + 12, yPos + 3);
    
    itemsInCurrentRow++;
    xPos += 45; // Spacing between items
    
    // Move to next row if needed
    if (itemsInCurrentRow >= maxItemsPerRow) {
      xPos = 20;
      yPos += 15; // Increased row spacing to prevent overlap
      itemsInCurrentRow = 0;
    }
  });

  // Issues Table with enhanced styling
  if (results.issues?.issues && results.issues.issues.length > 0) {
    const tableData = results.issues.issues.map(issue => [
      issue.type.replace('_', ' ').toUpperCase(),
      issue.severity.toUpperCase(),
      issue.message,
      issue.component.replace(project.project_name + ':', ''),
      issue.line ? issue.line.toString() : '-'
    ]);

    // Calculate table start position based on coverage section only (quality gate is now in top-right)
    let tableStartY = 95 + summaryHeight + 10;
    if (results.coverage && results.coverage.component && results.coverage.component.measures.length > 0) {
      tableStartY += 25; // Add space for coverage section
    }
    
    // Custom table styling with color-coded severity
    doc.autoTable({
      startY: tableStartY, // Dynamic position based on coverage section
      head: [['Issue Type', 'Severity', 'Issue Details', 'File Location', 'Line']],
      body: tableData,
      styles: {
        fontSize: 9, // Slightly smaller font for better fit
        cellPadding: 3,
        lineColor: [200, 200, 200],
        lineWidth: 0.5,
        overflow: 'linebreak', // Enable text wrapping
        halign: 'left',
        valign: 'top',
      },
      pageBreak: 'auto', // Enable automatic page breaks
      rowPageBreak: 'avoid', // Avoid breaking rows across pages
      tableWidth: 'wrap', // Allow table to use full width
      margin: { left: 10, right: 10 }, // Center the table with margins
      showHead: 'everyPage', // Show header on every page
      headStyles: {
        fillColor: [66, 139, 202], // Blue header
        textColor: 255,
        fontStyle: 'bold',
        fontSize: 11,
      },
      bodyStyles: {
        fontSize: 8, // Smaller font for better fit
        cellPadding: 4, // Increased padding
        overflow: 'linebreak',
        halign: 'left',
        valign: 'top',
        minCellHeight: 12, // Increased minimum row height
        lineColor: [200, 200, 200],
        lineWidth: 0.5,
      },
      columnStyles: {
        0: { cellWidth: 25 }, // Issue Type
        1: { cellWidth: 25 }, // Severity
        2: { cellWidth: 65, overflow: 'linebreak', halign: 'left' }, // Issue Details - significantly increased
        3: { cellWidth: 65, overflow: 'linebreak', halign: 'left' }, // File Location - significantly increased
        4: { cellWidth: 15 }, // Line
      },
      didDrawCell: (data: any) => {
        // Color code severity column
        if (data.column.index === 1 && data.row.index >= 0) {
          const severity = data.cell.text[0].toLowerCase();
          let color: number[];
          
          switch (severity) {
            case 'critical':
            case 'blocker':
              color = [220, 53, 69]; // Red
              break;
            case 'major':
              color = [255, 193, 7]; // Orange
              break;
            case 'minor':
              color = [255, 235, 59]; // Yellow
              break;
            case 'info':
              color = [13, 202, 240]; // Blue
              break;
            default:
              color = [108, 117, 125]; // Gray
          }
          
          // Set background color for severity cell
          doc.setFillColor(color[0], color[1], color[2]);
          doc.rect(data.cell.x, data.cell.y, data.cell.width, data.cell.height, 'F');
          
          // Set text color to white for better contrast
          doc.setTextColor(255, 255, 255);
          doc.setFont('helvetica', 'bold');
          doc.text(data.cell.text[0], data.cell.x + 3, data.cell.y + data.cell.height / 2 + 1);
          
          // Reset text color
          doc.setTextColor(0, 0, 0);
          doc.setFont('helvetica', 'normal');
        }
      },
      didDrawPage: (data: any) => {
        // Add page numbers
        const pageCount = doc.getNumberOfPages();
        const currentPage = data.pageNumber;
        doc.setFontSize(8);
        doc.setTextColor(100, 100, 100);
        doc.text(`Page ${currentPage} of ${pageCount}`, 20, doc.internal.pageSize.height - 10);
        doc.setTextColor(0, 0, 0);
      },
      didParseCell: (data: any) => {
        // Ensure proper text wrapping for long content
        if (data.column.index === 2) { // Issue Details
          if (typeof data.cell.text === 'string' && data.cell.text.length > 80) {
            // Split long text into multiple lines for better wrapping
            const words = data.cell.text.split(' ');
            const lines = [];
            let currentLine = '';
            
            for (const word of words) {
              if ((currentLine + word).length > 80) {
                if (currentLine) lines.push(currentLine.trim());
                currentLine = word;
              } else {
                currentLine += (currentLine ? ' ' : '') + word;
              }
            }
            if (currentLine) lines.push(currentLine.trim());
            
            data.cell.text = lines.join('\n');
          }
        } else if (data.column.index === 3) { // File Location
          if (typeof data.cell.text === 'string' && data.cell.text.length > 70) {
            // For file paths, split at directory separators for better readability
            const pathParts = data.cell.text.split('/');
            if (pathParts.length > 3) {
              // Split long paths at logical points
              const lines = [];
              let currentLine = '';
              
              for (const part of pathParts) {
                if ((currentLine + '/' + part).length > 70) {
                  if (currentLine) lines.push(currentLine.trim());
                  currentLine = part;
                } else {
                  currentLine += (currentLine ? '/' : '') + part;
                }
              }
              if (currentLine) lines.push(currentLine.trim());
              
              data.cell.text = lines.join('\n');
            }
          }
        }
      }
    });
  } else {
    // Calculate position for "No issues found" text based on coverage section only (quality gate is now in top-right)
    let noIssuesY = 95 + summaryHeight + 20;
    if (results.coverage && results.coverage.component && results.coverage.component.measures.length > 0) {
      noIssuesY += 25; // Add space for coverage section
    }
    
    doc.setFontSize(12);
    doc.text('No issues found in this scan.', 20, noIssuesY);
  }

  // Enhanced Footer
  const pageCount = doc.getNumberOfPages();
  for (let i = 1; i <= pageCount; i++) {
    doc.setPage(i);
    
    // Footer background
    doc.setFillColor(248, 249, 250);
    doc.rect(0, doc.internal.pageSize.height - 25, 210, 25, 'F');
    
    // Footer text
    doc.setFontSize(8);
    doc.setFont('helvetica', 'italic');
    doc.setTextColor(100, 100, 100);
    doc.text('Generated by SonarCute Code Check Report', 20, doc.internal.pageSize.height - 15);
    doc.text(`Generated on: ${currentDate}`, 20, doc.internal.pageSize.height - 10);
    
    // Reset text color
    doc.setTextColor(0, 0, 0);
  }

  // Save the PDF
  const fileName = `${project.project_name}_issues_report_${new Date().toISOString().split('T')[0]}.pdf`;
  doc.save(fileName);
};
