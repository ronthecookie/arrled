
// How many leds in your strip?
#define NUM_LEDS 60

// For led chips like WS2812, which have a data line, ground, and power, you just
// need to define DATA_PIN.  For led chipsets that are SPI based (four wires - data, clock,
// ground, and power), like the LPD8806 define both DATA_PIN and CLOCK_PIN
// Clock pin only needed for SPI based chipsets when not using hardware SPI
#define DATA_PIN 3
// Pin to sense PC power (comment it out to disable)
#define SENSE_PIN 4

// You might have to also change the code in main.cpp for different strips.
#define LED_TYPE WS2812B
