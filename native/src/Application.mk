APP_BUILD_SCRIPT := src/Android.mk
APP_ABI          := arm64-v8a
APP_CFLAGS       := -Wall -DNDEBUG -O3 -fomit-frame-pointer -flto
APP_LDFLAGS      := -flto -Wl,-icf=all,--lto-O3,-s,-x,--gc-sections,--no-undefined
APP_CPPFLAGS     := -std=c++23 -O3 -flto -DNDEBUG
APP_STL          := none
APP_PLATFORM     := android-23
APP_THIN_ARCHIVE := true
APP_STRIP_MODE   := --strip-all
APP_SUPPORT_FLEXIBLE_PAGE_SIZES := true

ifdef B_CRT0

# Disable all security and debugging features
APP_CFLAGS       +=	-fno-unwind-tables -fno-asynchronous-unwind-tables -fno-stack-protector -U_FORTIFY_SOURCE
# Override output folder to make sure all dependencies are rebuilt with new CFLAGS
NDK_APP_OUT      := ./obj/nolibc

endif

# Busybox should use a newer libc.a
ifdef B_BB
APP_PLATFORM     := android-26
ifeq ($(OS),Windows_NT)
APP_SHORT_COMMANDS := true
endif
endif
