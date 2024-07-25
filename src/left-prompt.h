#if defined(_WIN32)
/* WinSock2.h must be put before iphlpapi.h. */
#include <WinSock2.h>
#include <ws2tcpip.h> /* For inet_ntop declaration and IDE autocompletion.
                         It is not necessary for compilation. */
#include <iphlpapi.h>
/* Windows.h must be put after WinSock2.h. */
#include <Windows.h>
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "iphlpapi.lib")
#else
#if defined(__APPLE__)
#include <IOKit/ps/IOPSKeys.h>
#include <IOKit/ps/IOPowerSources.h>
#else
#include <fcntl.h>
#endif
#include <arpa/inet.h>
#include <ifaddrs.h>
#include <net/if.h>
#include <sys/statvfs.h>
#include <unistd.h>
#endif
#include "common.h"
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <tmk.h>

#if defined(__linux__)
#define BATTERY "/sys/class/power_supply/BAT0"
#endif

#if defined(_WIN32)
static void getPWDPath(char **utf8PWD, wchar_t **utf16PWD, size_t *length);
static size_t findLastPathSeparator(bool isWide, const void *path,
                                    size_t length);
static void findGitRoot(const wchar_t *utf16PWD, size_t pwdLength,
                        char **gitRoot, size_t *gitRootLength,
                        size_t *gitRootLastSeparatorOffset);
static void writeAdministratorRole(bool isAdministrator);
static void writeLastExitCode(int lastExitCode);
#else
static void getPWDPath(char **pwd, size_t *length);
static size_t findLastPathSeparator(const char *path, size_t length);
static void findGitRoot(const char *pwd, size_t pwdLength, char **gitRoot,
                        size_t *gitRootLength,
                        size_t *gitRootLastSeparatorOffset);
static void writeAdministratorRole(void);
static void writeLastExitCode(void);
#endif
static void writeVirtualEnv(void);
static void writePath(const char *pwd, const char *gitRoot,
                      size_t gitRootLastSeparatorOffset);
static void writeGitBranch(const char *gitRoot, size_t gitRootLength);
static void writeIPV4Address(void);
static void writeBatteryStatus(void);
static void writeDiskUse(void);
static void writeCalendar(struct tm *localTime);
static void writeClock(struct tm *localTime);
static void writeCommandLineSeparator(uint16_t totalColumns);
static void writeModulesSeparator(uint16_t totalColumns);
static void throwError(const char *format, ...);
static void *allocateHeapMemory(size_t size);
