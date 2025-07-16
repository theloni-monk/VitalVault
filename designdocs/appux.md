# App UX design doc

## Guiding principles:

Note that performance is not the highest priority at first

### Ease of use

#### Tutorial-free
Want users to be able to operate without a tutorial. Heavy use of iconography, as few words as possible.

#### Accessible
Users should not have to be able to understand english or american idioms / symbolism to use the software.

### Security
Password and/or biometric integration at login.

### No-nonsense: a tool, not your friend
This is more a philosophical point. I believe that technology shouldn't pretend that it "knows" you or cares about how you feel. The goal of this software is to help people retrive their medical information, it shouldn't do anything else.

There has been a big movement towards "friendly design" but I don't think this is a good paradigm in general. This doesn't mean we are making brutalist designs or something, just that the app shouldn't smile at you.

## UI Pages
### Splash-screen

#### Phases: 
##### First-boot: 
Loading code and text/img resources.

##### Store verification: 
Check each file in the store for tampering. Just a CRC is fine.


##### Model loading:
Verify model weights against a hash as you load them into the NPU high-bandwidth memory.

### About
Information on app, mission, credit, how to use, and links to other pages. Language option.

### Injest & Organize
file-folder style interface. Users can make folders upload documents to those folders. One should be able to search only within a given folder.

Tagging can allow for linking things across folders.

### Read
PDF & image renderer. Needs to handle decryption elegantly. Users should be able to leave notes or comments to the metadata to assist retrival.

### Find
Scope is given in props from organization page.

By default, search will rank order all documents in scope. Searches lists should be able to be narrowed by subsequent queries.

Documents should be presented as they are retrived. Probably want to use some sort of async walker-matcher service.

## Style Guide - still under discussion
Creme-colored background for easy navigation and high contrast.

White search bar.

blue-grey icons?
