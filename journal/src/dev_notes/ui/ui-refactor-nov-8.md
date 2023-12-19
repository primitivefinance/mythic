
# Improving maintainability

Right now the App renders a Screen which renders components. These components pipe messages upwards, and wraps it with the parent's event enum variant. This can get pretty confusing / hard to debug.

App components could break due to changes and we wouldn't know, because we don't have robust tests to make sure each component is behaving as expected.

Styling is not in one place, which can lead to discrepancies in the layout or look of the app.

Loading async things like fonts or the client connection to anvil, etc. is not handled in a clear or straightforward way, it's just in creating a new example screen.

No straightforward way to call async functions without piping the message all the way back to the app.

File/data output management is non-existent. How can we reliably save data in a directory?

## Launch process
- Launcher starts, handles logic to build the data directory, returns message install if good
- Installer starts from app
- Installer iterates through install steps and processes it
- Installer emits "Exit"
- Loader picks up off exit, starts loading
- Loads config into app, emits Synced
- Saves state into App, calls Run
- If state app & message run, call app.update
- Wrap app messages with run to process them running
- View method calls View on the State::Loading,Installer,App,Launcher, wraps with respective message

need to make this fast to debug!

App should have flags that are explicitly defined in some App Config.

Logger (tracing crate) should be handled in the App, App should store the State and Logger of it.