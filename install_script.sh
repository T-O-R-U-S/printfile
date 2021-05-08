echo 'Cloning repo...'
git clone https://github.com/T-O-R-U-S/printfile.git -q
cd printfile
echo 'Compiling binary...'
rustc src/main.rs -o pf
echo 'Installing binary...'
sudo mv pf /usr/local/bin/pf
echo 'Done!'