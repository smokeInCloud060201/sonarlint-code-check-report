## Issue Fixed: SonarQube API Request Format

### Problem
The application was sending JSON data to SonarQube API endpoints, but SonarQube requires **form-encoded data** for project creation and token generation endpoints.

### Error Message
```
"error":"Failed to create project in SonarQube: Failed to create project: {\"errors\":[{\"msg\":\"The 'project' parameter is missing\"}]}"
```

### Root Cause
The code was using `.json()` method with reqwest, but SonarQube's `/api/projects/create` endpoint expects form-encoded parameters.

### Solution Applied
Changed from JSON to form-encoded requests:

**Before:**
```rust
let request = CreateProjectRequest {
    name: project_name.to_string(),
    project: project_key.to_string(),
};

let response = self.client
    .post(&url)
    .header("Authorization", ...)
    .json(&request)  // ❌ Wrong format
    .send()
    .await?;
```

**After:**
```rust
let params = [
    ("project", project_key),
    ("name", project_name),
];

let response = self.client
    .post(&url)
    .header("Authorization", ...)
    .form(&params)  // ✅ Correct format
    .send()
    .await?;
```

### Files Modified
- `api/src/sonarqube/client.rs`
  - Updated `create_project()` method to use form-encoded data
  - Updated `create_project_token()` method to use form-encoded data

### Next Steps
1. **Stop the running application** (if currently running)
2. **Rebuild the application**:
   ```bash
   cargo build --release
   ```
3. **Restart the application**:
   ```bash
   cargo run
   ```
4. **Test again** with the same curl command

### Expected Result
The API should now successfully:
1. Create the project in SonarQube
2. Generate a project token
3. Store project configuration in database
4. Return complete project details including the token
