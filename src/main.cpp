#include <Arduino.h>
#include <FastLED.h>
#include <EEPROM.h>
#include "config.hpp"

#define EEPROM_SENSERGB_INDEX 0
#define EEPROM_SENSE_ENABLE_INDEX 3
const char help[] = "\f\f\fwelcome to ArrLED! (https://ronthecookie.me)\ncommands fmt: 0xff <inst byte> <args>\n0xfb-$r-$g-$b show color on all\nff-$i-$r-$g-$b set led\nfe-$r-$g-$b  store default in eeprom\nfc read default from eeprom\npossibly outdated src: https://gist.github.com/ronthecookie/8cfa30f5af8ab7640fc2f7c6491aec9e\n";
CRGB leds[NUM_LEDS];
bool senseEnable = true;

void setup()
{
	///
	///
	/// Change me!!
	FastLED.addLeds<LED_TYPE, DATA_PIN, GRB>(leds, NUM_LEDS);
	///
	///
	///

	Serial.begin(9600);
	EEPROM.begin();
	#ifdef SENSE_PIN
	pinMode(SENSE_PIN, INPUT);
	senseEnable = EEPROM.read(EEPROM_SENSE_ENABLE_INDEX);
#endif
}

int safeRead()
{
	while (!Serial.available())
	{
	}
	return Serial.read();
}

void loop()
{
	#ifdef SENSE_PIN
	static bool lastSenseRead = false;
	// pc power handle
	if (senseEnable) {
		bool sense = digitalRead(SENSE_PIN);
		if (!lastSenseRead && sense)
		{
			// read rgb from eeprom and show
			int r = EEPROM.read(EEPROM_SENSERGB_INDEX);
			int g = EEPROM.read(EEPROM_SENSERGB_INDEX + 1);
			int b = EEPROM.read(EEPROM_SENSERGB_INDEX + 2);
			FastLED.showColor(CRGB(r, g, b));
		}
		else if (!sense && lastSenseRead)
		{
			FastLED.showColor(CRGB(0, 0, 0));
		}
		lastSenseRead = sense;
	}
	#endif
	// serial handle
	int sig = Serial.read();
	if (sig != 0xff && sig != -1)
	{
		Serial.println(help);
		return;
	}
	else if (sig != 0xff)
		return;
	int inst = safeRead();
	if (inst == 0xff)
	{
		// set led
		int led = safeRead();
		int r = safeRead();
		int g = safeRead();
		int b = safeRead();
		if (led >= NUM_LEDS) {
			Serial.print("err");
			return;
		}
		leds[led] = CRGB(r, g, b);
		FastLED.show();
	}
	else if (inst == 0xfe)
	{
		// set default
		int r = safeRead();
		int g = safeRead();
		int b = safeRead();
		EEPROM.write(EEPROM_SENSERGB_INDEX, r);
		EEPROM.write(EEPROM_SENSERGB_INDEX + 1, g);
		EEPROM.write(EEPROM_SENSERGB_INDEX + 2, b);
	}
	else if (inst == 0xfc)
	{
		// read default
		int r = EEPROM.read(EEPROM_SENSERGB_INDEX);
		int g = EEPROM.read(EEPROM_SENSERGB_INDEX + 1);
		int b = EEPROM.read(EEPROM_SENSERGB_INDEX + 2);
		Serial.write(r);
		Serial.write(g);
		Serial.write(b);
	}
	else if (inst == 0xfb)
	{
		// show color on all
		int r = safeRead();
		int g = safeRead();
		int b = safeRead();
		FastLED.showColor(CRGB(r, g, b));
	}
	else if (inst == 0xfa) {
		// sense on/off
		bool toggle = safeRead() != 0x00;
		senseEnable = toggle;
		EEPROM.write(EEPROM_SENSE_ENABLE_INDEX, toggle);
	} else if (inst == 0xf9) {
		Serial.write(NUM_LEDS);
	}
	else
	{
		Serial.print("err");
	}
}