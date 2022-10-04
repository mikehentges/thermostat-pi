This project is a result of me searching for something to build to exercise my understanding of the Rust programming language. As I started to learn Rust, I looked for an intermediate-level text - something to move me from "I understand the syntax" to "I understand how to be productive. I ran across an excellent book by Luca Palmieri: [Zero to Production in Rust](https://www.zero2prod.com/). The book walks through the building of a non-trivial, production-quality web application in Rust. Many useful concepts are built out and illustrated, including:

- Building a production-quality web server application in Rust.
- Telemetry – getting traces/logs distributed to a central location and managing error handling across several different API stacks.
- Implementing end-user authentication – production-grade user authentication, including encryption and protection from common hacking methods.
- SQL interaction – Postgres, in this case, including migrations.
- External API usage – calling external REST-based services.
- HTML templates – producing web pages for end-user interaction from a server.
- Session management – Redis in this case, but a strategy for keeping track of end-user state.

After working through the book, I felt I had a decent handle on how this stuff works. As a bonus, I haven’t done a lot of production-level web server development in other languages – and seeing the normal challenges in this environment solved was interesting. But I’ve never claimed to be proficient in a tool until I’ve used it to build something. About this time, an outside inspiration popped up – I wanted a way to control a thermostat in a detached workshop remotely. The heater for my workshop is a simple 2-wire control – not nearly as complicated as a home furnace. A Google Nest (link) is overkill, I’ve done some programming for Arduino and Raspberry Pi devices in the past, so this project was born.

In laying out the architecture for my new project, I decided on the following initial scope:

1. I’m only worried about heating – so a single control and simplified logic for either turning on the heat or turning it off.
2. For the main control platform, the Raspberry Pi is an easy choice. It runs a Unix OS, has Wi-Fi networking support, and has easy access to the physical devices needed for temperature measurement and relays to work as a thermostat.
3. I would use Rust as the development language and take advantage of the following Rust features:
<ol type="a">
<li>Cross-compile to a native executable (ARM processor).</li>
<li>Use the Actix-Web framework to expose REST interfaces for remotely setting the thermostat and retrieving the current temperature.</li>
<li>Utilize the Tokio run time for a multi-threaded application that can read temperatures, decide whether to turn the thermostat on or off, receive commands to set the thermostat, and send data externally.</li>
<li>Gain programmatic access to physical devices on the Raspberry Pi for reading temperature and controlling a relay to turn the heater on or off.</li></ol>

4. I wanted external visibility into how the thermostat was working. I decided to push data to the cloud instead of keeping it on the device so that an external “watcher” can send an alarm if something isn’t working correctly. I wouldn’t want a thermostat not to turn on when it’s supposed to (freezing things is bad) or stay on too long (100 degrees is also bad and a potential fire hazard).
5. I chose a REST interface for controlling the thermostat – making it easy to connect something to it later. I might make a Flutter app for my phone (more new things to figure out!) or a web interface.
6. I chose to address security through physical network access and firewalls and not implement a security layer on the API. I am saving implementing security for the 2.0 version, which keeps the initial development cleaner and easier.
7. I would follow the main principles from Zero to Production in organizing the project and utilize many of the same external Rust packages. I liked the coding style and choices made in the book and consider it a best practices reference.

I intended this to be a hobby project – not a commercial production-ready implementation. When I install this for actual use, I will have a backup system to ensure nothing bad happens. Things work in the project’s current state – but I don’t profess to have the system hardened. More testing is needed before I’ll trust this with anything important.

The following diagram depicts the system architecture for the solution:

<img src="https://res.cloudinary.com/dbzsk4ytb/image/upload/c_scale,w_800/v1664843918/blog-images/sys-arch.drawio_rrv2ju.png" alt="DTMF IVR" />

The subsystems of the application are as follows:

- **Shared Data** – This holds the current state of the application and is accessed from the other subsystems as needed. Each subsystem runs on independent threads, so we wrap the shared data to protect it when multiple threads try to read/set its data independently.
- **Web (HTTP) Interface** – We run an actix-web server, allowing for multiple HTTP clients. This interface allows external access to the thermostat – giving it the “wireless” functionality we desire. These interfaces enable setting the thermostat value (the desired minimum temperature) and retrieving the current temperature and thermostat setting.
- **Read Current Temperature** – An independent thread polls the temperature sensor periodically. Each time it reads a temperature value, it changes the internal state and then pushes those values to the external cloud storage.
- **Set Thermostat** – Another independent thread polls the Shared Data to see if the thermostat should be on or off. When the temperature is below the thermostat value, the external control relay is set on – else, it is set off. A minimum time on/off threshold avoids thrashing, so we store the control relay change time in the Shared Data, which becomes part of the logic for determining the relay state.
- **Telemetry** – All tracing and error messages are funneled through a Telemetry layer and pushed to an external collector for visibility.
- **Cloud Data Storage** – We’re using AWS as our cloud provider and utilizing a Lambda function (written in Rust!) to receive HTTP messages from the application and store them in a DynamoDB table for external processing. The cloud database is our repository of historical information on how the temperature and thermostat settings have changed over time. Eventually, a monitor against this data will alert us if something goes wrong.
- **End-User UI** – We need a way to control the thermostat and see its current state. This UI interacts with the application via HTTP to get/set data. It also can retrieve historical data from the cloud for display.

I’m still fiddling with things and working on a GUI client. But for now, it works!
