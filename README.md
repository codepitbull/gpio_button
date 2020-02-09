# What's this
The raspberry Pi 3 (and I think 2 alöso had these) has USB-ports where the power can be disabled and reenabled using 
this nice CLI-tool: [uhubctl](https://github.com/mvp/uhubctl).

Using my tool you can tie the power control to a GPIO-button.

I use this to enable/disable a little screen I have connected to my raspberry.

## Building
I use cross for building everything. 

```cargo install cross```

```cross build --target=arm-unknown-linux-gnueabihf```

## Running
```Usage: ./gpio_button <pin> <delayinseconds>```

**pin** is the number of the GPIO-pin.

**delayinseconds** specifies the number of seconds after which the display will be deactivated. 
