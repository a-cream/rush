pub const TokenType = enum {
    command,
};

pub const Token = union(TokenType) {
    command: []const u8,
};
