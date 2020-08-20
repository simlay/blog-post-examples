.PHONY: cargo-bundle install run

bundle:
	cargo bundle --example uikit --target x86_64-apple-ios

bundle-run: install
	xcrun simctl launch --console booted com.github.simlay.uikit

bundle-install: bundle
	xcrun simctl install booted target/x86_64-apple-ios/debug/examples/bundle/ios/uikit.app

bundle-setup:
	cargo install cargo-bundle --git https://github.com/burtonageo/cargo-bundle


run: install
	xcrun simctl launch --console booted com.rust.use-uikit-example

install: xcodebuild
	 xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/use_uikit_example.app

xcodebuild: generate
	xcodebuild -scheme use_uikit_example -configuration Debug -destination 'platform=iOS Simulator,name=iPhone SE (2nd generation),OS=13.6' -derivedDataPath build

generate: project.yml ios-src/Info.plist ios-src/main.m ios-src/bindings.h
	xcodegen

clean:
	rm -r build
	cargo clean
