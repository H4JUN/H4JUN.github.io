# Do I need to delete thebranch?
sudo trunk build --release
git add ./ && git commit -m "Update dist"
git subtree push --prefix dist origin gh-pages

