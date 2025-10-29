# Testing Mock Data in Driver.js Tour

## Quick Test Steps:

1. **Open your browser** and navigate to the application
2. **Open Developer Console** (F12)
3. **Go to Application tab** → Local Storage
4. **Delete the `sonar-tour-completed` key** (if it exists)
5. **Refresh the page**
6. **Check the console** for these messages:
   ```
   🚀 Starting tour - injecting mock data...
   ✅ Mock data injected, isTourActive: true
   🎬 Starting Driver.js tour...
   📊 Fetching projects (should be mock data)...
   🔄 Tour is active, fetching mock data...
   🔍 getMockProjects called, isTourActive: true
   📦 Mock data: [{...}] // Should show the "Sample Java Project"
   ```

7. **You should see:**
   - A "Sample Java Project" card in the UI
   - The Driver.js tour overlay active
   - Console logs confirming mock data is being used

## What You Should See:

- **Project Card**: "Sample Java Project" with Java language badge
- **Project Path**: "/path/to/sample-project"
- **Sonar Host**: "http://localhost:9000"
- **Tour Popover**: Sonar image and instructions

## If Mock Data Doesn't Appear:

1. **Check the console logs** - Do you see the injection messages?
2. **Check the tour state** - Is `isTourActive: true`?
3. **Clear localStorage** - Try clearing all data and refreshing
4. **Check the network tab** - Make sure no errors are occurring

## Debugging:

If the logs show `isTourActive: false` when it should be true, then:
- The mock data isn't being injected properly
- The timing might be off

## Expected Behavior:

✅ **First visit**: Shows mock project with tour
✅ **After tour completes**: Mock project disappears, shows real data
✅ **Subsequent visits**: No mock data (tour already completed)

## Testing Commands:

```bash
# Clear tour completion
localStorage.removeItem('sonar-tour-completed')

# Check tour state
console.log('isTourActive:', isTourCurrentlyActive())

# Force show mock data
injectMockDataForTour()
console.log('Mock projects:', getMockProjects())
```
