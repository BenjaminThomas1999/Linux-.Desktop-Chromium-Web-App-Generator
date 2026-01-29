This is a basic Rust script that writes .desktop files to /home/$user/.local/share/applications.

The .desktop files follow a very basic format so the user only has to type in the:
  1. Title
  2. Description
  3. URL
  4. icon

Improvements:

  1. Use fzf to select icon from a list of installed icons.
  2. Use the system whoami command to find the home folder, or use tilda.
  3. Add a config file, for the user to adjust defaults.
