#!/bin/sh


# ==============================================
# Script: generate_tokens.sh
# Description: Generate two SonarQube tokens via API
# ==============================================

API_URL="http://sonarcuteapi:8080/api/admin-token"
USERNAME="admin"
PASSWORD="admin"
SONAR_HOST_URL="http://sonarqube:9000"

# --- First call: GLOBAL_ANALYSIS_TOKEN ---
echo " Generating GLOBAL_ANALYSIS_TOKEN..."
curl --location "$API_URL" \
  --header 'Content-Type: application/json' \
  --data "{
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"token_name\": \"admin_analisys_token_global\",
    \"token_type\": \"GLOBAL_ANALYSIS_TOKEN\",
    \"sonar_host_url\": \"$SONAR_HOST_URL\"
  }"

echo -e "\n GLOBAL_ANALYSIS_TOKEN request completed.\n"

# --- Second call: USER_TOKEN ---
echo " Generating USER_TOKEN..."
curl --location "$API_URL" \
  --header 'Content-Type: application/json' \
  --data "{
    \"username\": \"$USERNAME\",
    \"password\": \"$PASSWORD\",
    \"token_name\": \"admin_analisys_token_user\",
    \"token_type\": \"USER_TOKEN\",
    \"sonar_host_url\": \"$SONAR_HOST_URL\"
  }"

echo -e "\n USER_TOKEN request completed.\n"

echo " All token generation requests finished!"
