const std = @import("std");

const Shell = struct {
    prompt: []const u8,

    fn new(prompt: []const u8) Shell {
        return Shell{ .prompt = prompt };
    }

    fn ask(self: Shell, buf: []u8) ![]const u8 {
        const stdin = std.io.getStdIn().reader();
        const stdout = std.io.getStdOut().writer();

        _ = try stdout.print("{s}", .{self.prompt});

        const input = (try stdin.readUntilDelimiterOrEof(buf, '\n')) orelse {
            return error.Null;
        };

        return input;
    }
};

pub fn run() !void {
    const shell = Shell.new("> ");

    while (true) {
        var buf: [1024]u8 = undefined;
        const input = try shell.ask(&buf);

        std.debug.print("{s}\n", .{input});
    }
}
