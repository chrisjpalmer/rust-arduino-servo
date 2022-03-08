chown root:root /home/chrisjpalmer/.cargo/bin/ravedude
sudo chmod u+s /home/chrisjpalmer/.cargo/bin/ravedude

#https://superuser.com/questions/1150768/how-do-i-grant-access-dev-console-to-an-executable

#https://github.com/microsoft/WSL/issues/4322
#https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/

usbipd wsl attach --busid 1-11
usbipd list