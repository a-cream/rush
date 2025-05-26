const std = @import("std");
const types = @import("types.zig");

const U8Buffer = struct {
    data: [100]u8 = undefined,
    len: usize = 0,

    pub fn push(self: *U8Buffer, bytes: []const u8) !void {
        if (self.len + bytes.len > self.data.len) return error.Overflow;
        @memcpy(self.data[self.len .. self.len + bytes.len], bytes);
        self.len += bytes.len;
    }

    pub fn asSlice(self: *U8Buffer) []const u8 {
        return self.data[0..self.len];
    }
};

pub const Lexer = struct {
    data: []const u8,
    index: usize = 0,

    pub fn next(self: *Lexer) ?u8 {
        if (self.index >= self.data.len) return null;
        const val = self.data[self.index];
        self.index += 1;
        return val;
    }

    pub fn peek(self: *Lexer) ?u8 {
        if (self.index >= self.data.len) return null;
        return self.data[self.index];
    }

    pub fn lex(self: *Lexer) !std.ArrayList(types.Token) {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer arena.deinit();

        const allocator = arena.allocator();

        var tokens = std.ArrayList(types.Token).init(allocator);

        while (self.next()) |c| {
            if ((c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z')) {
                var buf = U8Buffer{};
                buf.push(&[_]u8{c}) catch return error.BufferPushFail;

                while (self.next()) |ch| {
                    if ((ch >= 'a' and ch <= 'z') or (ch >= 'A' and ch <= 'Z')) {
                        buf.push(&[_]u8{ch}) catch return error.BufferPushFail;
                    } else {
                        break;
                    }
                }

                try tokens.append(types.Token{ .Arg = buf.asSlice() });
            }
        }

        return tokens;
    }
};
