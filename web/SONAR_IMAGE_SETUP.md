# Sonar Guide Image Setup

To add Sonar's image to the tour, please follow these steps:

## 1. Add the Image File
Place the Sonar guide image in the `web/public/` directory with the filename `sonar-guide.png`

## 2. Image Requirements
- **Format**: PNG or JPG
- **Size**: Recommended 200x200px or larger (will be resized to 50-60px in tour)
- **Aspect Ratio**: Square (1:1) works best for the circular display
- **Quality**: High quality for crisp display

## 3. File Location
```
web/public/sonar-guide.png
```

## 4. Tour Integration
The tour is already configured to use the image at `/sonar-guide.png`. Each tour step will display:
- Sonar's image (circular, 50-60px)
- Personalized message from Sonar
- Feature explanation

## 5. Tour Features with Image
- **Welcome Step**: Larger image (60px) with introduction
- **Feature Steps**: Smaller image (50px) with contextual messages
- **Visual Appeal**: Circular border with subtle shadow
- **Responsive**: Works on all screen sizes

## 6. Example Tour Step
```
┌─────────────────────────────────────────┐
│ 👋 Welcome to SonarQube Code Check!    │
│                                         │
│ [🖼️ Sonar Image] Hi! I'm Sonar, your   │
│                 code quality guide.      │
│                 Let me show you around! │
│                                         │
│ First, you can create new projects by   │
│ clicking this "Add New Project" button. │
│                                         │
│ [Previous] [Next] [Skip Tour]           │
└─────────────────────────────────────────┘
```

Once you add the image file, the tour will automatically display Sonar's friendly face in each step, making the experience much more engaging and personal!
