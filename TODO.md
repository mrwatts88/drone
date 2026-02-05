# TODO

- [ ] hardcode IMU values and compute PID
- [ ] motor mixing calculations
- [ ] handle panic
- [ ] put CAD drawings in this repo
- [ ] order electronics
- [ ] read from IMU over SPI
- [ ] onboard computer to relay bluetooth to uart control commands
- [ ] ground control application
- [ ] finish frame 3D model
- [ ] print frame
- [ ] assemble
- [ ] tune
- [ ] fly

# DONE

- [x] initial frame 3D model
- [x] Basic MCU bring-up
- [x] UART communication (ST-LINK virtual COM)
- [x] RTT debug output
- [x] why refcell in mutex? why not data directly or cell?
- [x] UART framing in buffer A and writing to buffer B
- [x] validate control frame in main and write to intent struct
- [x] timer to read from intent struct
- [x] send data to ESCs over pwm

