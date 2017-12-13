TODO: 
* add a web form to the index where users can manually input new pastes. Accept the form at POST /. Use format and/or rank to specify which of the two POST / routes should be called.
* Add a new route, GET /<id>/<lang> that syntax highlights the paste with ID <id> for language <lang>. If <lang> is not a known language, do no highlighting. Possibly validate <lang> with FromParam.
* persist in a DB instead of file (diesel+postgres)

WIP:
* Dispatch a thread before launching Rocket in main that periodically cleans up idling old pastes in upload/.

Done:
* add config file
* Use the local module to write unit tests for your pastebin.
* Add a PUT /<id> route that allows a user with the key for <id> to replace the existing paste, if any.
* Support deletion of pastes by adding a new DELETE /<id> route. Use PasteID to validate <id>.
* Limit the upload to a maximum size. If the upload exceeds that size, return a 206 partial status code. Otherwise, return a 201 created status code.
* Set the Content-Type of the return value in upload and retrieve to text/plain.
* Return a unique “key” after each upload and require that the key is present and matches when doing deletion. Use one of Rocket’s core traits to do the key validation.