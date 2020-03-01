# Copyright 2019 sukawasatoru
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

SHELL = /bin/sh
.SUFFIXES:
.PHONY: all release clean distclean check test deploy setup storybook
GIT = git
NPM = npm
NPX = npx
RM = rm -f
RMRF = rm -rf
touch = touch $(1)
FORCE_DEPLOY =

ifeq ($(OS),Windows_NT)
	SHELL = cmd
	RM = del /Q
	RMRF = rmdir /S /Q
	touch = type nul > $(1)
endif

all: release

release: setup
	$(NPM) run build

clean:
	-$(RMRF) dist

distclean: clean
	-$(RMRF) node_modules

check: test
# 	TODO

test:
# 	TODO
	-$(NPM) test

deploy:
ifeq ($(CI)$(FORCE_DEPLOY),)
	@echo If you want to deploy, should set variable "FORCE_DEPLOY=true"
	exit 1
endif
	$(GIT) add -A
	$(GIT) diff --staged --exit-code
	$(MAKE) clean
	$(MAKE) release
	$(NPX) gh-pages --dist dist

setup: node_modules/setup-dummy

storybook: setup
	$(NPM) run build-storybook

node_modules/setup-dummy: package.json
ifeq (npm,$(NPM))
ifneq ($(CI),)
	$(NPM) ci
else
	$(NPM) install
endif
else
	$(NPM) install
endif
	$(call touch,node_modules/setup-dummy)
