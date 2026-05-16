#include <cmath>
#include <cstdint>
#include <iostream>

#include <fstream>
#include <sstream>
void log(std::string text);
void error(std::string text);
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <variant>
#include <vector>

#include "code/json.hpp"
using json = nlohmann::json;

enum class TokenType {
  KEY,
  CPP_BLOCK,
  NUMBER,
  STRING,
  SCOPE,
  SYMBOL,
  ASSIGN,
  MEMBER,
  BRACKET,
  SEMICOLON,
  COMMA,
  VALUE
};

struct Token {
  std::string value;
  TokenType type;
  int line;
};

TokenType GetTypeFromString(const std::string &typeStr) {
  static const std::map<std::string, TokenType> typeMap = {
      {"KEY", TokenType::KEY},         {"CPP_BLOCK", TokenType::CPP_BLOCK},
      {"NUMBER", TokenType::NUMBER},   {"STRING", TokenType::STRING},
      {"SCOPE", TokenType::SCOPE},     {"SYMBOL", TokenType::SYMBOL},
      {"ASSIGN", TokenType::ASSIGN},   {"MEMBER", TokenType::MEMBER},
      {"BRACKET", TokenType::BRACKET}, {"SEMICOLON", TokenType::SEMICOLON},
      {"COMMA", TokenType::COMMA},     {"VALUE", TokenType::VALUE}};

  auto it = typeMap.find(typeStr);
  if (it != typeMap.end()) {
    return it->second;
  }

  error("Unknown token type: " + typeStr);

  return TokenType::VALUE;
}
#include <filesystem>
void print(std::string text);
void println(std::string text);
std::string read();
std::string load(std::string FilePath);
int32_t sysCall(std::string command);
float _sqrt(float a);
void log(std::string text);
void error(std::string text);
std::vector<Token> tokenize(std::string FileName);
int32_t main();

void print(std::string text) { std::cout << text; }

void println(std::string text) { std::cout << text << "\n"; }

std::string read() {
  std::string text;
  std::cin >> text;
  return text;
}

std::string load(std::string FilePath) {
  std::ifstream file(FilePath);

  if (!file.is_open()) {
    error("File can't be opened: " + FilePath);
  }

  std::stringstream buffer;
  buffer << file.rdbuf();
  std::string data = buffer.str();

  return data;
}

int32_t sysCall(std::string command) { return system(command.c_str()); }

float _sqrt(float a) { return std::sqrt(a); }

void log(std::string text) { std::cout << "Log: " << text << "\n"; }

void error(std::string text) {
  std::cout << "Error: " << text << "\n";
  __debugbreak();
  abort();
}

std::vector<Token> tokenize(std::string FileName) {
  std::string command = ("python assets/lexer.py " + FileName);
  int32_t out = sysCall(command);
  if ((out != 0)) {
    error("Lexer has errored");
  }
  std::string jsonFile = load("assets/in.json");
  nlohmann::json data = nlohmann::json::parse(jsonFile);
  std::vector<Token> tokens;
  if (data.is_array()) {
    int32_t i = 0;
    while ((data.size() > i)) {
      Token t;
      nlohmann::json item = data[i];
      t.value = item.value("value", "");
      t.line = item.value("line", 0);
      std::string typeStr = item.value("type", "");
      t.type = GetTypeFromString(typeStr);
      tokens.push_back(t);
      i++;
    }
  }
  print("Loaded " + std::to_string(tokens.size()) + " tokens.");
  return tokens;
}

int32_t main() {
  std::filesystem::path project_root = std::filesystem::current_path();
  std::filesystem::current_path(std::filesystem::temp_directory_path());
  std::filesystem::current_path(project_root);
  println("COMPILING");
  std::vector<Token> tokens = tokenize("assets/code/test.sm");
}
