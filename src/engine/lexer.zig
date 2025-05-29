const std = @import("std");

var buffer: [1000]u8 = undefined;
var fba = std.heap.FixedBufferAllocator.init(&buffer);
const allocator = fba.allocator();

pub const TokenKind = enum {
    Atomic,
    Land,
    EOF,
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
            'a'...'z', 'A'...'Z' => {
                const i = start;
                while (start < source.len and
                    ((source[start] >= 'a' and source[start] <= 'z') or
                        (source[start] >= 'A' and source[start] <= 'Z')))
                {
                    start += 1;
                }
                try tokens.append(Token{ .kind = TokenKind.Atomic, .value = source[i..start] });
            },
            '&' => {
                    if (start + 1 < source.len and source[start + 1] == '&') {
                    try tokens.append(Token{ .kind = TokenKind.Land, .value = source[start .. start + 2] });
                    start += 2;
                } else {
                    return error.InvalidChar;
                }
            },
            ' ', '\n', '\t' => start += 1,
            else => {
                return error.InvalidChar;
            },
        }
    }

    try tokens.append(Token{ .kind = TokenKind.EOF, .value = source[0..1] });
    return tokens;
}
