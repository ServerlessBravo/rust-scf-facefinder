const wasm = require("./pkg/rust_scf_facefinder");

exports.main_handler = async (event, context, callback) => {
  var body = event['body']
  var finder_result = wasm.process_event(body);

  return {
    isBase64Encoded: false,
    statusCode: 200,
    headers: { 'Content-Type': 'application/json' },
    body: finder_result,
  }
}

