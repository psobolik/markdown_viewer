# Uninstall
Here's how to uninstall this application, assuming it was manually installed as described in `DEPLOY.md`.
1. Uninstall the application from the desktop
```shell
$ xdg-desktop-menu uninstall mdvw.desktop
```
2. Edit `~/.config/mimetypes.list` file to remove `mdvw.desktop` from the `text/markdown` handlers.
3. Remove the app folder and its contents
```shell
$ rm -r ~/bin/markdown-viewer/
```
4. Edit `~/.profile` to remove the code that adds `~/bin/markdown-viewer` to the `PATH`

5. Remove the app's icon
```shell
$ rm ~/.local/share/icons/mdvw.svg
```
