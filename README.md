# display-switch

This command memorizes and switches between multiple display placement settings.

## Installation

Clone the repository and run the following command (cargo, brew and displayplacer commands are required).

```
make
sudo make install
```

## Usage

The `dsw` command performs the following operations in the following order.

1. if the current display placement settings have not been saved, name and save them
2. switch to the next placement setting after the current display placement in the save order

The display placement settings are saved in `$HOME/.display-switch.json`, so if you want to save the placement settings, delete them directly from here.
