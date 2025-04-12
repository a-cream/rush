#ifndef INTERPRETER_LEXER_H
#define INTERPRETER_LEXER_H

typedef enum {
    ARG,
} TokenType;

typedef struct {
    TokenType type;
    const char *value;
} Token;

typedef struct {
    const char *src;
    char current_char;
} Lexer;

void Lexer_init(Lexer *self, const char *src);
Token* Lexer_lex(Lexer *self);

#endif // !INTERPRETER_LEXER_H
