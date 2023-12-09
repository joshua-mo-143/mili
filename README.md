# mili: A self-hostable link shortener with QR code gen and analytics.

mili aims to be a self-hostable link shortener that's easy to setup and use with QR code generation and analytics.

## How to Deploy

### Shuttle
- Use `cargo shuttle init --from joshua-mo-143/mili`. Follow the prompt.
- Use `cargo shuttle deploy`. Wait around a bit until it's finished deploying.
- That's it!

### Regular Dockerfile deployment
- You will need to make sure to set up the Dockerfile and make sure to pass in the following environment variables:
	- `DATABASE_URL` (the Postgres database URL)
	- `DOMAIN_URL` (the Domain URL)
- There is currently nothing to allow working with TLS currently, so you are almost certainly going to need to deploy to somewhere like Railway that has the service behind their own reverse proxy. 

## Feature Roadmap 
### Basic Functionality
- [x] Link shortening
- [x] Custom links
- [x] QR Code generation
- [x] Superimpose logos on QR codes
- [x] Basic overall hit counter 

### Long Term
- [ ] Vector image QR code option
- [ ] Make QR code more customisable
- [ ] Optional auth 
- [ ] Proper resizing for QR code logos
- [ ] More indepth analytics per link (privacy respecting)
