---
layout: default
title: "The rule of the big three (and a half) - Source code"
parent: "C++"
#math: mathjax
date: 2014-12-22 00:11:38
last_modified_date: 2020-05-03 22:42:28
---


# The rule of the big three (and a half)
{: .no_toc }

## Introduction
{: .no_toc }

This is my first article in English on this blog so please be forgiving.

I recently read a C++ article about [The rule of the big three (and a half)](https://blog.feabhas.com/2014/12/the-rule-of-the-big-three-and-a-half-resource-management-in-c/) and I took the time to make sure the code snippets of the article works on my PC.

Indeed the evil is always in the details and it usually help me a lot to practice when I'm learning something new. Anyway, here is what I came with.

Each source code is complete, meaning you should be able to copy paste it directly. I only tested the code under Visual Studio Community 2013. Obviously the source code below only make sense if you read the article and play with your compiler at the same time. In few places I added comments that may help the reader.

This is a zip with the solution and the projects : [RuleOf3.5](./assets/RuleOf3.5.zip)


## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}



## Snippet 1 :

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
                                                                                // The "new" above in the members initialization list create the Socket
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;                                                               // Destroy the Socket
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {

  {                                                                             // The block allow to see messages from ctor & dtor
    SocketManager mgr{ 2002, 0x7F000001 };
    mgr.send("Hello World.");
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 2

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager mgr1{ 2002, 0x7F000002 };
    SocketManager mgr2{ mgr1 };                                                 // Explicit copy contruction
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 3

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;                                                    // Do NOT enable this line in order to declare "SocketManager mgr;" in main(). If so no assertion failed at runtime
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager mgr1{ 2002, 0x7F000002 };
    SocketManager mgr2 = mgr1;                                                  // Calls cpy ctor NOT operator=
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 4

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;                                                  // Do NOT enable this line in order to declare "SocketManager mgr;" in main(). If so no assertion failed at runtime
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

void func(SocketManager mgr) {                                                  // Note : pass by value
  mgr.send("Hello World from func()");
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager mgr{ 2002, 0x7F000002 };
    func(mgr);
    mgr.send("Hello World from main()");
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 5

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

SocketManager make_SocketManager(const IPAddress& addr, const Port& port) {
  SocketManager temp{ addr, port };
  return temp;
}

SocketManager NRV_make_SocketManager(const IPAddress& addr, const Port& port) {
  return SocketManager{ addr, port };                                           // NRVO : Named Return Value Optimization
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager mgr1 = make_SocketManager(2002, 21);
    SocketManager mgr2 = NRV_make_SocketManager(2002, 21);
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 6

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

void preamble(SocketManager mgr) {                                              // Note : pass by value
  mgr.send("Hello World");
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager socketMgr{ 2002, 0x7F000001 };
    preamble(socketMgr);
    socketMgr.send("Hello World");
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 7

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;                                                  // Do NOT enable this line in order to declare "SocketManager mgr;" in main(). If so no assertion failed at runtime
  SocketManager(const SocketManager& src);                                      // Cpy ctor
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

SocketManager::SocketManager(const SocketManager& src) : ip{ src.ip }, pSocket{ nullptr } {
                                                                                // Remember - use the MIL above (MIL = members initialization list)
  if (src.pSocket != nullptr) {
    pSocket = new Socket{ *src.pSocket };                                       // Copy conctruct the Socket object
  }
  pSocket->open();
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {                                                                    // Run in Debug mode. Generates an assertion failed

  {
    SocketManager mgr1{ 2002, 0x7F000001 };;
    SocketManager mgr2 = mgr1;                                                  // cpy ctor

    SocketManager mgr3{ 1001, 0x7F000011 };;
    mgr3 = mgr1;                                                                // This is where the problem is

    mgr1.send("Hello World");
    mgr2.send("Goodbye cruel world");
    mgr3.send("Am I still here?");
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 8

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;
  SocketManager(const SocketManager& src);                                      // Cpy ctor
  SocketManager& operator=(const SocketManager& rhs);                           // Assignment operator
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
  void swap(SocketManager& rhs);
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

SocketManager::SocketManager(const SocketManager& src) : ip{ src.ip }, pSocket{ nullptr } {
  if (src.pSocket != nullptr) {
    pSocket = new Socket{ *src.pSocket };
  }
  pSocket->open();
}

SocketManager& SocketManager::operator=(const SocketManager& rhs) {
  SocketManager temp{ rhs };                                                    // Cpy construct temp. Generates the third "Socket opened" message
  swap(temp);
  return *this;
}

void SocketManager::swap(SocketManager& rhs) {
  std::swap(this->ip, rhs.ip);                                                  // Swap each attribute
  std::swap(this->pSocket, rhs.pSocket);
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {

  {
    SocketManager mgr1{ 2002, 0x7F000001 };
    SocketManager mgr2{ 1441, 0x7F00007F };

    mgr2 = mgr1;
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 9

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;
  SocketManager(const SocketManager& src);                                      // Cpy ctor
  SocketManager& operator=(SocketManager rhs);                                  // Pass by value and no longer const
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
  void swap(SocketManager& rhs);
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

SocketManager::SocketManager(const SocketManager& src) : ip{ src.ip }, pSocket{ nullptr } {
  if (src.pSocket != nullptr) {
    pSocket = new Socket{ *src.pSocket };
  }
  pSocket->open();
}

SocketManager& SocketManager::operator= (SocketManager rhs) {                   // Pass by value. No longer const
  swap(rhs);
  return *this;
}

void SocketManager::swap(SocketManager& rhs) {
  std::swap(this->ip, rhs.ip);
  std::swap(this->pSocket, rhs.pSocket);
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {

  {
    SocketManager mgr1{ 2002, 0x7F000001 };
    SocketManager mgr2{ 1441, 0x7F00007F };

    mgr2 = mgr1;
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

## Snippet 10

```cpp
#include <iostream>
#include <limits>

using namespace std;

using Port      = int;
using IPAddress = int;

class Socket {
public:
  Socket(Port p);
  void open();
  void close();
  void write(const char *buf);
private:
  Port port;
};

Socket::Socket(Port p) : port{ p } {
}

void Socket::open() {
  cout << "Socket opened" << endl;
}

void Socket::close() {
  cout << "Socket closed" << endl;
}

void Socket::write(const char *buf) {
  cout << "Message sent : " << buf << endl;
}

class SocketManager {
public:
  SocketManager(IPAddress addr, Port p);
  ~SocketManager();
  //SocketManager() = default;
  SocketManager& operator=(SocketManager rhs) = delete;                         // No need to define these functions
  SocketManager(const SocketManager& src) = delete;
  void send(const char *str) const;
private:
  IPAddress ip;
  Socket *pSocket;
  void swap(SocketManager& rhs);
};

SocketManager::SocketManager(IPAddress addr, Port p) : ip{ addr }, pSocket{ new Socket{ p } } {
  pSocket->open();
}

SocketManager::~SocketManager() {
  pSocket->close();
  delete pSocket;
}

void SocketManager::swap(SocketManager& rhs) {
  std::swap(this->ip, rhs.ip);
  std::swap(this->pSocket, rhs.pSocket);
}

void SocketManager::send(const char *str) const {
  pSocket->write(str);
}

int main() {                                                                    // Does not compile. Indeed Cpy ctor is "deleted"

  {
    SocketManager mgr1{ 2002, 0x7F000001 };
    SocketManager mgr2{ 1441, 0x7F00007F };

    mgr2 = mgr1;
  }

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

