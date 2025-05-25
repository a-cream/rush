const std = @import("std");

const Shell = struct {
    prompt: []const u8,

    fn new(prompt: []const u8) Shell {
        return Shell{ .prompt = prompt };
    }

    fn input(self: Shell) ![]const u8 {
        const stdout_file = std.io.getStdOut().writer();
        var bw = std.io.bufferedWriter(stdout_file);
        const stdout = bw.writer();

        try stdout.print("{s}", .{self.prompt});

        try bw.flush();

        const stdin = std.io.getStdIn().reader();
        var buf: [100]u8 = undefined;

        const user_input = try stdin.readUntilDelimiterOrEof(&buf, '\n');

        return user_input orelse error.EndOfFile;
    }
};

pub fn run() !void {
    const shell = Shell.new("> ");

    while (true) {
        const input = try shell.input();
        std.debug.print("{s}\n", .{input});
    }
}
