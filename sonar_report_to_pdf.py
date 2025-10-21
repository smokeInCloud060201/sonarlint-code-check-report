import sys
import json
from pathlib import Path
import pdfkit
import html as html_module  # For escaping HTML

# wkhtmltopdf configuration
config = pdfkit.configuration(
    wkhtmltopdf=r"C:\Program Files\wkhtmltopdf\bin\wkhtmltopdf.exe"
)

# Get project name from command-line argument
project_name = sys.argv[1] if len(sys.argv) > 1 else "default_project"
base_dir = Path(__file__).parent

html_path = base_dir / f"{project_name}_sonar_report.html"
pdf_path = base_dir / f"{project_name}_sonar_report.pdf"
json_path = base_dir / f"{project_name}_sonar_issues_report.json"

print(f"JSON Path: {json_path}")
if not json_path.exists():
    print(f"{json_path} not found!")
    exit(1)

# Load JSON
with open(json_path, "r", encoding="utf-8") as f:
    data = json.load(f)

issues = data.get("issues", [])

# Severity color map
severity_colors = {
    "BLOCKER": "#e74c3c",
    "CRITICAL": "#e67e22",
    "MAJOR": "#f1c40f",
    "MINOR": "#2ecc71",
    "INFO": "#3498db"
}

# Build HTML
html = f"""<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>SonarQube Report For {project_name}</title>
<style>
body {{ font-family: Arial, sans-serif; margin: 40px; }}
h1 {{ text-align: center; color: #2c3e50; }}
div.table-container {{ overflow-x: auto; }}
table {{
    border-collapse: collapse;
    width: 100%;
    table-layout: fixed;  /* Fix column widths */
    margin-top: 20px;
}}
th, td {{
    border: 1px solid #ddd;
    padding: 8px;
    font-size: 12px;
    text-align: left;
    word-wrap: break-word;  /* Wrap long content */
}}
th {{
    background-color: #4CAF50;
    color: white;
}}
tr:nth-child(even) {{ background-color: #f9f9f9; }}
.severity {{
    color: white;
    font-weight: bold;
    padding: 3px 6px;
    border-radius: 5px;
    text-transform: uppercase;
    display: inline-block;
}}
th:nth-child(1), td:nth-child(1) {{ width: 90px; }}   /* Type */
th:nth-child(2), td:nth-child(2) {{ width: 70px; }}   /* Severity */
th:nth-child(5), td:nth-child(5) {{ width: 50px; }}   /* Line */
</style>
</head>
<body>
<h1>SonarQube Issue Report For {project_name}</h1>
<div class="table-container">
<table>
<tr>
  <th>Type</th>
  <th>Severity</th>
  <th>Message</th>
  <th>File</th>
  <th>Line</th>
</tr>
"""

# Build table rows
for issue in issues:
    severity = issue.get("severity", "").upper()
    color = severity_colors.get(severity, "#7f8c8d")
    html += "<tr>"
    html += f"<td style='width:90px'>{issue.get('type','')}</td>"
    html += f"<td style='width:70px'><span class='severity' style='background-color:{color}'>{severity}</span></td>"
    html += f"<td>{html_module.escape(issue.get('message',''))}</td>"
    html += f"<td>{issue.get('component','')}</td>"
    html += f"<td style='width:50px'>{issue.get('line') or ''}</td>"
    html += "</tr>"

html += "</table></div></body></html>"

# Write HTML file
html_path.write_text(html, encoding="utf-8")
print(f"HTML file generated at: {html_path.resolve()}")

# Generate PDF
pdfkit.from_file(str(html_path), str(pdf_path), configuration=config)
print(f"PDF generated at: {pdf_path.resolve()}")
