from machine import Pin, I2C
import time

commands = {
    0:  bytes.fromhex("518403100000a8"),
    10: bytes.fromhex("51840310000aa2"),
    20: bytes.fromhex("518403100014bc"),
    30: bytes.fromhex("51840310001eb6"),
    40: bytes.fromhex("51840310002880"),
    50: bytes.fromhex("5184031000329a"),
    60: bytes.fromhex("51840310003c94"),
    70: bytes.fromhex("518403100046ee"),
    80: bytes.fromhex("518403100050f8"),
    90: bytes.fromhex("51840310005af2"),
    100:bytes.fromhex("518403100064cc")
}

brightness = 0

i2c = I2C(0, scl=Pin(1), sda=Pin(0), freq=100_000)

p4 = Pin(4, Pin.IN, Pin.PULL_UP)

def wait_pin_change(pin):
    active = 0
    while active < 20:
        if pin.value() == 0:
            active += 1
        else:
            active = 0
        time.sleep_ms(1)

while True:

    wait_pin_change(p4)

    brightness = (brightness + 10) % 110
    print(f"Write brightness {brightness}")
    
    try:
        i2c.writeto(0x37, commands[brightness])
    except OSError:
        print("I/O Error")

    time.sleep_ms(300)