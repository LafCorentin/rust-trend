pub fn sanitize_response(body: &str, pos: usize) -> &str {
    &body[pos..]
}
