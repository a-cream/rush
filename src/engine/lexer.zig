const std = @import("std");

var buffer: [1000]u8 = undefined;

var fba = std.heap.FixedBufferAllocator.init(&buffer);
const allocator = fba.allocator();

pub const TokenKind = enum {
    command,
};

pub const Token = struct {
    kind: TokenKind,
    value: []u8,
};

pub fn lex(source: []u8) !std.ArrayList(Token) {
    var tokens = std.ArrayList(Token).init(allocator);

    var start: usize = 0;
    while (start < source.len) {
        const current = source[start];

        switch (current) {
            'a'...'z' => {
                const i = start;
                while (start < source.len and source[start] >= 'a' and source[start] <= 'z') {
                    start += 1;
                }
                try tokens.append(Token{ .kind = TokenKind.command, .value = source[i..start] });
            },
            ' ', '\n', '\t' => start += 1,
            else => {
                return error.InvalidChar;
            },
        }
    }

    return tokens;
}
