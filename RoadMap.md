# RoadMap

## Introduction
Welcome to the RoadMap for the GUI Game Save Manager project! This document outlines the planned features, milestones, and tasks for the development of the application.

## Features
The GUI Game Save Manager aims to provide the following key features:

- Graphical user interface for easy interaction.
- Backup and restore functionality for game saves.
- Support for multiple games and save files.
- Automatic and manual backup options.
- User-friendly interface for managing backups and restores.
- Customizable settings for backup frequency, destination, etc.

## Milestones
The development of Oxidized Save Manager will be divided into the following milestones:


### Milestone 1: Backup and Restore Functionality
- [x] Build the core data structs for Settings, Game, and Save
- [ ] Implement core logic for backup and restore operations.
- [ ] Integrate with file system for saving and loading game files.
- [ ] Test backup and restore functionality with sample game saves.

### Milestone 2: Project Setup and UI Design
- [ ] Set up project structure and dependencies.
- [ ] Design user interface layout and components.
- [ ] Implement basic UI elements (e.g., buttons, file explorer).

### Milestone 3: Multi-game Support
- [ ] Extend the application to support multiple games.
- [ ] Implement game selection feature in the UI.
- [ ] Ensure compatibility with various game save formats.

### Milestone 4: Pull game library/images from Steam
- [X] Pull list of games from SteamDB
- [X] Pull out the game icons from the Steam db
- [ ] Present images and games
- [ ] Possibly show a compatibility indicator.
- [ ] Possibly show a ProtonDB indicator.

### Milestone 5: Advanced Features
- [ ] Add automatic backup scheduling feature.
- [ ] Implement settings menu for customization options.
- [ ] Enhance UI with progress indicators, notifications, etc.

### Milestone 6: Testing and Optimization
- [ ] Conduct thorough testing for reliability and performance.
- [ ] Optimize code for efficiency and resource usage.
- [ ] Address any reported bugs or issues.

### Milestone 7: Documentation and Release
- [ ] Write comprehensive documentation for users and developers.
- [ ] Prepare for the official release of the application.

## Tasks
Here's a breakdown of tasks to be completed for each milestone:

### Milestone 1: Backup and Restore Functionality
| Task                              | Issue/PR |         Description                           | Status      |
|-----------------------------------|----------|-----------------------------------------------|-------------|
| Build out core Data structs       | [#1](https://github.com/HirschBerge/oxidized_saves/pull/1)| Build out the Settings, Game, and Save structs| Merged     |
| Implement backup logic            | [#2](https://github.com/HirschBerge/oxidized_saves/pull/2)     | Develop code for backing up game saves        | In Progress |
| Implement restore logic           | [#2](https://github.com/HirschBerge/oxidized_saves/pull/2) | Develop code for restoring game saves         | In Progress |
| Test backup and restore           | [#2](https://github.com/HirschBerge/oxidized_saves/pull/2) | Verify functionality with sample game saves   | In Progress |

### Milestone 2: Project Setup and UI Design
| Task                              | Issue/PR          | Description                          | Status      |
|-----------------------------------|------|---------------------------------------------------|-------------|
| Set up project structure          |  N/A | Create directories and configuration files        | Not Started |
| Design UI layout                  |  N/A | Sketch wireframes and layout design               | Not Started |
| Implement basic UI elements       |  N/A | Create buttons, file explorer, etc.               | Not Started |

<!-- Add more tasks for subsequent milestones -->

## Conclusion
The RoadMap provides a structured plan for the development of the GUI Game Save Manager. As progress is made on each milestone and task, updates will be reflected in this document.

