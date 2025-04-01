# Install
The Dioxus bundler doesn't work correctly for Linux desktop deployment at this time, so I installed this app manually.

First, I built a release version:
```shell
$ dx build --release
```
Then I copied the executable *and* the assets to my bin folder.
```shell
$ rsync --recursive --mkpath ~/Development/dioxus/markdown_viewer/target/dx/mdvw/release/linux/app/ ~/bin/markdown-viewer/
```
I added `~/bin/markdown-viewer` to my path in `.profile` so the app can be invoked as `mdvw`.
```shell
if [ -d "$HOME/bin/markdown-viewer" ] ; then
    case ":${PATH}:" in 
        *:"$HOME/bin/markdown-viewer":*)
            ;;
        *)
            PATH="$PATH:$HOME/bin/markdown-viewer"
            ;;
    esac
fi
export PATH
```
# File-type Handler
To associate Markdown with the app, I installed the icon and desktop menu files:
```shell
$ cp ~/Development/dioxus/markdown_viewer/Deploy/mdvw.svg ~/.local/share/icons/
$ xdg-desktop-menu install --novendor ~/Development/dioxus/markdown_viewer/Deploy/mdvw.desktop
```

I also made `mdvw` the default handler for Markdown files:
```shell
$ xdg-mime query default text/markdown 
```