# LIGHT Package Management Suite
# install.sh
# Written by Jamason Davis - 2021

#!/bin/sh
arg=$1

prep () {
	[ ! -d "./light-tmp" ] && mkdir ./light-tmp
	cd ./light-tmp
}


softclean () {
	[ -d "../light-tmp" ] && rm -rf ../light-tmp
}

cleanup () {
	softclean
	[ -f "/usr/bin/light" ] && sudo rm /usr/bin/light
	[ -d "/etc/light" ] && sudo rm -r /etc/light
}

download () {
	if command -v git
	then
		git clone https://github.com/hurbOS/LIGHT.git
	else 
		echo "Git is not installed, Please install Git before continuing. [EC-11]"
		cleanup
		exit
	fi
}

build () {
	if command -v cargo
	then
		cd ./LIGHT
		cargo build
	else
		echo "Cargo is missing, please ensure Rust is installed before continuing. [EC-21]"
		cleanup
		exit
	fi
}

install () {
	[ ! -d "/etc/light" ] && sudo mkdir /etc/light
	if [ $? -ne 0 ] 
	then
		echo "Error making /etc/light [EC-31]"
		cleanup
		exit
	fi
	sudo cp ./target/debug/light /usr/bin/light
	if [ $? -ne 0 ] 
	then
		echo "Error moving the LIGHT binary. [EC-32]"
		cleanup
		exit
	fi
	#sudo cp ./conf/* /etc/light/
	#if [ $? -ne 0 ] 
	#then
	#	echo "Error copying config files [EC-33]"
	#	cleanup
	#	exit
	#fi
}

echo "Welcome to the LIGHT automated installer, Cloning most recent source of LIGHT from GitHub"
prep
if [ $? -ne 0 ]
then
	echo "Error creating temporary directory. [EC-10]"
	cleanup
	exit
fi
download
if [ $? -ne 0 ]
then
	echo "Error cloning repository. [EC-20]"
	cleanup
	exit
fi
build
if [ $? -ne 0 ]
then
	echo "Error building LIGHT [EC-30]"
	cleanup
	exit
fi
install
if [ $? -ne 0 ]
then
	echo "Error installing LIGHT [EC-40]"
	cleanup
	exit
fi
softclean
echo "Welcome to LIGHT! Run 'light --help' for more information"
