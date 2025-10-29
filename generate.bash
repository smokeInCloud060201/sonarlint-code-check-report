#!/bin/bash

token=sqp_6c61e6dc2cf01510d398dba75c6a2717a9c59648
component=caspersky
page_size=500
project_name=$component

# Fetch first page to get total issues
curl -s -u $token: "http://localhost:9000/api/issues/search?componentKeys=$component&ps=$page_size&p=1" -o "${project_name}_issues_page_1.json"

# Extract total
total=$(jq '.total' "${project_name}_issues_page_1.json")
pages=$(( (total + page_size - 1) / page_size ))

echo "Total issues: $total → Total pages: $pages"

# Loop through all pages
for i in $(seq 1 $pages); do
  echo "Fetching page $i"
  curl -s -u $token: \
    "http://localhost:9000/api/issues/search?componentKeys=$component&ps=$page_size&p=$i" \
    -o "${project_name}_issues_page_$i.json"
done

# Merge all pages
jq -s '{ issues: map(.issues) | add }' ${project_name}_issues_page_*.json > ${project_name}_sonar_issues_report.json

# Cleanup
rm -f ${project_name}_issues_page_*.json

echo "All issues fetched and merged into ${project_name}_sonar_issues_report.json"

# Run Python automatically and pass project name
py sonar_report_to_pdf.py "$project_name"
