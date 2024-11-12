#include <Arduino.h>
#include <Wire.h>

#define PLUS PD0
#define MINUS PC0

#define ADDRESS 0x37

// Pre-computed commands for 0-100% brightness
uint8_t commands[11][7] = {
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x00, 0xa8},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x0a, 0xa2},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x14, 0xbc},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x1e, 0xb6},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x28, 0x80},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x32, 0x9a},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x3c, 0x94},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x46, 0xee},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x50, 0xf8},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x5a, 0xf2},
    {0x51, 0x84, 0x03, 0x10, 0x00, 0x64, 0xcc}
};

TwoWire Display_A = TwoWire((uint32_t)PC1, (uint32_t)PC2);
TwoWire Display_B = TwoWire((uint32_t)PC6, (uint32_t)PC5);

int8_t brightness = 5;
int8_t change = 0;

void set(uint8_t *command){
    Display_A.begin((uint32_t)PC1, (uint32_t)PC2);
    Display_A.beginTransmission(ADDRESS);
    Display_A.write(command, 7);
    Display_A.endTransmission(true);
    Display_A.end();

    Display_B.begin((uint32_t)PC6, (uint32_t)PC5);
    Display_B.beginTransmission(ADDRESS);
    Display_B.write(command, 7);
    Display_B.endTransmission(true);
    Display_B.end();
}

void setup(){
    pinMode(PLUS, INPUT_PULLUP);
    pinMode(MINUS, INPUT_PULLUP);
}

void loop(){
    int8_t new_change = digitalRead(PLUS) - digitalRead(MINUS);

    if (new_change == 0){
        change = 0;
        return;
    }

    // Detect press and hold
    if (change == new_change){
        change = new_change * 10;
    }
    else {
        change = new_change;
    }

    int8_t new_brightness = constrain(brightness + change, 0, 10);
    
    if(brightness != new_brightness) {
        brightness = new_brightness;
        set(commands[brightness]);
    }

    delay(300);
}
