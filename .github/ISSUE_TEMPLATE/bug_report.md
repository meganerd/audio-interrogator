---
name: Bug Report
about: Create a report to help us improve audio-interrogator
title: '[BUG] '
labels: ['bug']
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Run command '...'
2. With these audio devices connected '...'
3. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Actual output**
If applicable, paste the actual output from audio-interrogator:
```
Paste output here
```

**Environment (please complete the following information):**
 - OS: [e.g. Ubuntu 22.04, Fedora 38]
 - Rust version: [e.g. 1.70.0]
 - Audio-interrogator version: [e.g. 0.1.0]
 - Audio system: [e.g. PulseAudio, JACK, plain ALSA]

**Audio Hardware**
Please provide output of the following commands:
```bash
# List of cards
cat /proc/asound/cards

# ALSA devices
aplay -l
arecord -l

# Audio-interrogator card listing
audio-interrogator --list
```

**Additional context**
Add any other context about the problem here.

**Error logs**
If applicable, add any error messages or logs:
```
Paste error logs here
```
