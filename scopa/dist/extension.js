/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ([
/* 0 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(__webpack_require__(1));
const wasm_component_model_1 = __webpack_require__(2);
const tergo_1 = __webpack_require__(30);
async function activate(context) {
    console.log("tergo activated");
    const filename = vscode.Uri.joinPath(context.extensionUri, 'scopa.wasm');
    console.log(`Looking for the WASM under ${filename}`);
    const bits = await vscode.workspace.fs.readFile(filename);
    const module = await WebAssembly.compile(bits);
    const wasmContext = new wasm_component_model_1.WasmContext.Default();
    const instance = await WebAssembly.instantiate(module, {});
    wasmContext.initialize(new wasm_component_model_1.Memory.Default(instance.exports));
    const api = tergo_1.tergo._.exports.bind(instance.exports, wasmContext);
    vscode.languages.registerDocumentFormattingEditProvider("r", {
        provideDocumentFormattingEdits(document, _options, _token) {
            let documentText = document.getText();
            console.log(`Formatting the document:\n${documentText}`);
            return [
                vscode.TextEdit.replace(new vscode.Range(document.lineAt(0).range.start, document.lineAt(document.lineCount - 1).range.end), api.format(documentText)),
            ];
        },
    });
}
// This method is called when your extension is deactivated
function deactivate() {
    console.log("tergo deactivated");
}


/***/ }),
/* 1 */
/***/ ((module) => {

module.exports = require("vscode");

/***/ }),
/* 2 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
const ril_1 = __importDefault(__webpack_require__(3));
ril_1.default.install();
__exportStar(__webpack_require__(7), exports);


/***/ }),
/* 3 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
/// <reference path="../../typings/webAssemblyNode.d.ts" preserve="true"/>
const util_1 = __webpack_require__(4);
const worker_threads_1 = __webpack_require__(5);
const ral_1 = __importDefault(__webpack_require__(6));
const _ril = Object.freeze({
    TextEncoder: Object.freeze({
        create(encoding = 'utf-8') {
            return {
                encode(input) {
                    return Buffer.from(input ?? '', encoding);
                }
            };
        }
    }),
    TextDecoder: Object.freeze({
        create(encoding = 'utf-8') {
            return new util_1.TextDecoder(encoding);
        }
    }),
    console: console,
    timer: Object.freeze({
        setTimeout(callback, ms, ...args) {
            const handle = setTimeout(callback, ms, ...args);
            return { dispose: () => clearTimeout(handle) };
        },
        setImmediate(callback, ...args) {
            const handle = setImmediate(callback, ...args);
            return { dispose: () => clearImmediate(handle) };
        },
        setInterval(callback, ms, ...args) {
            const handle = setInterval(callback, ms, ...args);
            return { dispose: () => clearInterval(handle) };
        }
    }),
    Connection: Object.freeze({
        async createWorker(port, world, timeout) {
            if (port === undefined) {
                port = worker_threads_1.parentPort;
            }
            if (!(port instanceof MessagePort)) {
                throw new Error(`Expected MessagePort`);
            }
            const connection = await __webpack_require__.e(/* import() */ 1).then(__webpack_require__.bind(__webpack_require__, 29));
            return new connection.WorkerConnection(port, world, timeout);
        },
        async createMain(port) {
            if (!(port instanceof MessagePort) && !(port instanceof worker_threads_1.Worker)) {
                throw new Error(`Expected MessagePort or Worker`);
            }
            const connection = await __webpack_require__.e(/* import() */ 1).then(__webpack_require__.bind(__webpack_require__, 29));
            return new connection.MainConnection(port);
        }
    }),
    Worker: Object.freeze({
        getPort() {
            return worker_threads_1.parentPort;
        },
        getArgs() {
            return process.argv.slice(2);
        },
        get exitCode() {
            return process.exitCode;
        },
        set exitCode(value) {
            process.exitCode = value;
        }
    }),
    WebAssembly: Object.freeze({
        compile(bytes) {
            return WebAssembly.compile(bytes);
        },
        instantiate(module, imports) {
            return WebAssembly.instantiate(module, imports);
        }
    })
});
function RIL() {
    return _ril;
}
(function (RIL) {
    function install() {
        if (!ral_1.default.isInstalled()) {
            ral_1.default.install(_ril);
        }
    }
    RIL.install = install;
})(RIL || (RIL = {}));
if (!ral_1.default.isInstalled()) {
    ral_1.default.install(_ril);
}
exports["default"] = RIL;


/***/ }),
/* 4 */
/***/ ((module) => {

module.exports = require("util");

/***/ }),
/* 5 */
/***/ ((module) => {

module.exports = require("worker_threads");

/***/ }),
/* 6 */
/***/ ((__unused_webpack_module, exports) => {


/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
Object.defineProperty(exports, "__esModule", ({ value: true }));
let _ral;
function RAL() {
    if (_ral === undefined) {
        throw new Error(`No runtime abstraction layer installed`);
    }
    return _ral;
}
(function (RAL) {
    function install(ral) {
        if (ral === undefined) {
            throw new Error(`No runtime abstraction layer provided`);
        }
        _ral = ral;
    }
    RAL.install = install;
    function isInstalled() {
        return _ral !== undefined;
    }
    RAL.isInstalled = isInstalled;
})(RAL || (RAL = {}));
exports["default"] = RAL;


/***/ }),
/* 7 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.f32 = exports.char = exports.byte = exports.bool = exports.WasmContext = exports.VariantType = exports.Uint8ArrayType = exports.Uint32ArrayType = exports.Uint16ArrayType = exports.TupleType = exports.StaticMethodType = exports.ResultType = exports.ResultError = exports.ResourceType = exports.ResourceManagers = exports.ResourceManager = exports.ResourceHandleType = exports.Resource = exports.RecordType = exports.ReadonlyMemoryRange = exports.PackageType = exports.OwnType = exports.Options = exports.OptionType = exports.MethodType = exports.MemoryRange = exports.MemoryError = exports.Memory = exports.ListType = exports.InterfaceType = exports.Int8ArrayType = exports.Int32ArrayType = exports.Int16ArrayType = exports.FunctionType = exports.FlatTypeKind = exports.FlagsType = exports.EnumType = exports.DestructorType = exports.ConstructorType = exports.ComponentModelTypeKind = exports.ComponentModelTrap = exports.ComponentModelContext = exports.BorrowType = exports.BigUint64ArrayType = exports.BigInt64ArrayType = exports.BaseMemoryRange = exports.Alignment = exports.$main = exports.$imports = exports.$exports = void 0;
exports.RAL = exports.Connection = exports.wstring = exports.u8 = exports.u64 = exports.u32 = exports.u16 = exports.size = exports.s8 = exports.s64 = exports.s32 = exports.s16 = exports.result = exports.ptr = exports.option = exports.i64 = exports.i32 = exports.float64 = exports.float32 = exports.f64 = void 0;
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
const ral_1 = __importDefault(__webpack_require__(6));
exports.RAL = ral_1.default;
var componentModel_1 = __webpack_require__(8);
Object.defineProperty(exports, "$exports", ({ enumerable: true, get: function () { return componentModel_1.$exports; } }));
Object.defineProperty(exports, "$imports", ({ enumerable: true, get: function () { return componentModel_1.$imports; } }));
Object.defineProperty(exports, "$main", ({ enumerable: true, get: function () { return componentModel_1.$main; } }));
Object.defineProperty(exports, "Alignment", ({ enumerable: true, get: function () { return componentModel_1.Alignment; } }));
Object.defineProperty(exports, "BaseMemoryRange", ({ enumerable: true, get: function () { return componentModel_1.BaseMemoryRange; } }));
Object.defineProperty(exports, "BigInt64ArrayType", ({ enumerable: true, get: function () { return componentModel_1.BigInt64ArrayType; } }));
Object.defineProperty(exports, "BigUint64ArrayType", ({ enumerable: true, get: function () { return componentModel_1.BigUint64ArrayType; } }));
Object.defineProperty(exports, "BorrowType", ({ enumerable: true, get: function () { return componentModel_1.BorrowType; } }));
Object.defineProperty(exports, "ComponentModelContext", ({ enumerable: true, get: function () { return componentModel_1.ComponentModelContext; } }));
Object.defineProperty(exports, "ComponentModelTrap", ({ enumerable: true, get: function () { return componentModel_1.ComponentModelTrap; } }));
Object.defineProperty(exports, "ComponentModelTypeKind", ({ enumerable: true, get: function () { return componentModel_1.ComponentModelTypeKind; } }));
Object.defineProperty(exports, "ConstructorType", ({ enumerable: true, get: function () { return componentModel_1.ConstructorType; } }));
Object.defineProperty(exports, "DestructorType", ({ enumerable: true, get: function () { return componentModel_1.DestructorType; } }));
Object.defineProperty(exports, "EnumType", ({ enumerable: true, get: function () { return componentModel_1.EnumType; } }));
Object.defineProperty(exports, "FlagsType", ({ enumerable: true, get: function () { return componentModel_1.FlagsType; } }));
Object.defineProperty(exports, "FlatTypeKind", ({ enumerable: true, get: function () { return componentModel_1.FlatTypeKind; } }));
Object.defineProperty(exports, "FunctionType", ({ enumerable: true, get: function () { return componentModel_1.FunctionType; } }));
Object.defineProperty(exports, "Int16ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Int16ArrayType; } }));
Object.defineProperty(exports, "Int32ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Int32ArrayType; } }));
Object.defineProperty(exports, "Int8ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Int8ArrayType; } }));
Object.defineProperty(exports, "InterfaceType", ({ enumerable: true, get: function () { return componentModel_1.InterfaceType; } }));
Object.defineProperty(exports, "ListType", ({ enumerable: true, get: function () { return componentModel_1.ListType; } }));
Object.defineProperty(exports, "Memory", ({ enumerable: true, get: function () { return componentModel_1.Memory; } }));
Object.defineProperty(exports, "MemoryError", ({ enumerable: true, get: function () { return componentModel_1.MemoryError; } }));
Object.defineProperty(exports, "MemoryRange", ({ enumerable: true, get: function () { return componentModel_1.MemoryRange; } }));
Object.defineProperty(exports, "MethodType", ({ enumerable: true, get: function () { return componentModel_1.MethodType; } }));
Object.defineProperty(exports, "OptionType", ({ enumerable: true, get: function () { return componentModel_1.OptionType; } }));
Object.defineProperty(exports, "Options", ({ enumerable: true, get: function () { return componentModel_1.Options; } }));
Object.defineProperty(exports, "OwnType", ({ enumerable: true, get: function () { return componentModel_1.OwnType; } }));
Object.defineProperty(exports, "PackageType", ({ enumerable: true, get: function () { return componentModel_1.PackageType; } }));
Object.defineProperty(exports, "ReadonlyMemoryRange", ({ enumerable: true, get: function () { return componentModel_1.ReadonlyMemoryRange; } }));
Object.defineProperty(exports, "RecordType", ({ enumerable: true, get: function () { return componentModel_1.RecordType; } }));
Object.defineProperty(exports, "Resource", ({ enumerable: true, get: function () { return componentModel_1.Resource; } }));
Object.defineProperty(exports, "ResourceHandleType", ({ enumerable: true, get: function () { return componentModel_1.ResourceHandleType; } }));
Object.defineProperty(exports, "ResourceManager", ({ enumerable: true, get: function () { return componentModel_1.ResourceManager; } }));
Object.defineProperty(exports, "ResourceManagers", ({ enumerable: true, get: function () { return componentModel_1.ResourceManagers; } }));
Object.defineProperty(exports, "ResourceType", ({ enumerable: true, get: function () { return componentModel_1.ResourceType; } }));
Object.defineProperty(exports, "ResultError", ({ enumerable: true, get: function () { return componentModel_1.ResultError; } }));
Object.defineProperty(exports, "ResultType", ({ enumerable: true, get: function () { return componentModel_1.ResultType; } }));
Object.defineProperty(exports, "StaticMethodType", ({ enumerable: true, get: function () { return componentModel_1.StaticMethodType; } }));
Object.defineProperty(exports, "TupleType", ({ enumerable: true, get: function () { return componentModel_1.TupleType; } }));
Object.defineProperty(exports, "Uint16ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Uint16ArrayType; } }));
Object.defineProperty(exports, "Uint32ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Uint32ArrayType; } }));
Object.defineProperty(exports, "Uint8ArrayType", ({ enumerable: true, get: function () { return componentModel_1.Uint8ArrayType; } }));
Object.defineProperty(exports, "VariantType", ({ enumerable: true, get: function () { return componentModel_1.VariantType; } }));
Object.defineProperty(exports, "WasmContext", ({ enumerable: true, get: function () { return componentModel_1.WasmContext; } }));
Object.defineProperty(exports, "bool", ({ enumerable: true, get: function () { return componentModel_1.bool; } }));
Object.defineProperty(exports, "byte", ({ enumerable: true, get: function () { return componentModel_1.byte; } }));
Object.defineProperty(exports, "char", ({ enumerable: true, get: function () { return componentModel_1.char; } }));
Object.defineProperty(exports, "f32", ({ enumerable: true, get: function () { return componentModel_1.f32; } }));
Object.defineProperty(exports, "f64", ({ enumerable: true, get: function () { return componentModel_1.f64; } }));
Object.defineProperty(exports, "float32", ({ enumerable: true, get: function () { return componentModel_1.float32; } }));
Object.defineProperty(exports, "float64", ({ enumerable: true, get: function () { return componentModel_1.float64; } }));
Object.defineProperty(exports, "i32", ({ enumerable: true, get: function () { return componentModel_1.i32; } }));
Object.defineProperty(exports, "i64", ({ enumerable: true, get: function () { return componentModel_1.i64; } }));
Object.defineProperty(exports, "option", ({ enumerable: true, get: function () { return componentModel_1.option; } }));
Object.defineProperty(exports, "ptr", ({ enumerable: true, get: function () { return componentModel_1.ptr; } }));
Object.defineProperty(exports, "result", ({ enumerable: true, get: function () { return componentModel_1.result; } }));
Object.defineProperty(exports, "s16", ({ enumerable: true, get: function () { return componentModel_1.s16; } }));
Object.defineProperty(exports, "s32", ({ enumerable: true, get: function () { return componentModel_1.s32; } }));
Object.defineProperty(exports, "s64", ({ enumerable: true, get: function () { return componentModel_1.s64; } }));
Object.defineProperty(exports, "s8", ({ enumerable: true, get: function () { return componentModel_1.s8; } }));
Object.defineProperty(exports, "size", ({ enumerable: true, get: function () { return componentModel_1.size; } }));
Object.defineProperty(exports, "u16", ({ enumerable: true, get: function () { return componentModel_1.u16; } }));
Object.defineProperty(exports, "u32", ({ enumerable: true, get: function () { return componentModel_1.u32; } }));
Object.defineProperty(exports, "u64", ({ enumerable: true, get: function () { return componentModel_1.u64; } }));
Object.defineProperty(exports, "u8", ({ enumerable: true, get: function () { return componentModel_1.u8; } }));
Object.defineProperty(exports, "wstring", ({ enumerable: true, get: function () { return componentModel_1.wstring; } }));
var connection_1 = __webpack_require__(26);
Object.defineProperty(exports, "Connection", ({ enumerable: true, get: function () { return connection_1.Connection; } }));


/***/ }),
/* 8 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.VariantType = exports.FlagsType = exports.tuple = exports.TupleType = exports.RecordType = exports.Float64ArrayType = exports.Float32ArrayType = exports.BigUint64ArrayType = exports.Uint32ArrayType = exports.Uint16ArrayType = exports.Uint8ArrayType = exports.BigInt64ArrayType = exports.Int32ArrayType = exports.Int16ArrayType = exports.Int8ArrayType = exports.list = exports.ListType = exports.wstring = exports.char = exports.ptr = exports.size = exports.byte = exports.float64 = exports.float32 = exports.s64 = exports.s32 = exports.s16 = exports.s8 = exports.u64 = exports.u32 = exports.u16 = exports.u8 = exports.bool = exports.ResultError = exports.ComponentModelContext = exports.ComponentModelTypeKind = exports.FlatTuple = exports.f64 = exports.f32 = exports.i64 = exports.i32 = exports.FlatTypeKind = exports.Options = exports.Memory = exports.MemoryRange = exports.ReadonlyMemoryRange = exports.BaseMemoryRange = exports.MemoryError = exports.Alignment = exports.ComponentModelTrap = void 0;
exports.$main = exports.$exports = exports.$imports = exports.WasmContext = exports.PackageType = exports.InterfaceType = exports.OwnType = exports.BorrowType = exports.ResourceType = exports.ResourceHandleType = exports.MethodType = exports.StaticMethodType = exports.DestructorType = exports.ConstructorType = exports.FunctionType = exports.ResourceManagers = exports.ResourceManager = exports.Resource = exports.ResultType = exports.result = exports.OptionType = exports.option = exports.EnumType = void 0;
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
const uuid = __importStar(__webpack_require__(9));
const ral_1 = __importDefault(__webpack_require__(6));
// Type arrays are stored either little or big endian depending on the platform.
// The component model requires little endian so we throw for now if the platform
// is big endian. We can alternatively use data views in type arrays component
// model types to support big endian platforms.
const isLittleEndian = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;
if (!isLittleEndian) {
    throw new Error('Big endian platforms are currently not supported.');
}
class ComponentModelTrap extends Error {
    constructor(message) {
        super(message);
    }
}
exports.ComponentModelTrap = ComponentModelTrap;
var BigInts;
(function (BigInts) {
    const MAX_VALUE_AS_BIGINT = BigInt(Number.MAX_VALUE);
    function asNumber(value) {
        if (value > MAX_VALUE_AS_BIGINT) {
            throw new ComponentModelTrap('Value too big for number');
        }
        return Number(value);
    }
    BigInts.asNumber = asNumber;
    function max(...args) {
        return args.reduce((m, e) => e > m ? e : m);
    }
    BigInts.max = max;
    function min(...args) {
        return args.reduce((m, e) => e < m ? e : m);
    }
    BigInts.min = min;
})(BigInts || (BigInts = {}));
const utf8Decoder = (0, ral_1.default)().TextDecoder.create('utf-8');
const utf8Encoder = (0, ral_1.default)().TextEncoder.create('utf-8');
var Alignment;
(function (Alignment) {
    Alignment[Alignment["byte"] = 1] = "byte";
    Alignment[Alignment["halfWord"] = 2] = "halfWord";
    Alignment[Alignment["word"] = 4] = "word";
    Alignment[Alignment["doubleWord"] = 8] = "doubleWord";
})(Alignment || (exports.Alignment = Alignment = {}));
(function (Alignment) {
    function align(ptr, alignment) {
        return Math.ceil(ptr / alignment) * alignment;
    }
    Alignment.align = align;
    function getAlignment(ptr) {
        if (ptr % Alignment.doubleWord === 0) {
            return Alignment.doubleWord;
        }
        if (ptr % Alignment.word === 0) {
            return Alignment.word;
        }
        if (ptr % Alignment.halfWord === 0) {
            return Alignment.halfWord;
        }
        return Alignment.byte;
    }
    Alignment.getAlignment = getAlignment;
})(Alignment || (exports.Alignment = Alignment = {}));
const align = Alignment.align;
class MemoryError extends ComponentModelTrap {
    constructor(message) {
        super(message);
    }
}
exports.MemoryError = MemoryError;
class BaseMemoryRange {
    _memory;
    _ptr;
    _size;
    _alignment;
    _view;
    constructor(memory, ptr, size) {
        this._memory = memory;
        this._ptr = ptr;
        this._size = size;
        this._alignment = Alignment.getAlignment(ptr);
    }
    get memory() {
        return this._memory;
    }
    get ptr() {
        return this._ptr;
    }
    get size() {
        return this._size;
    }
    get alignment() {
        return this._alignment;
    }
    get view() {
        if (this._view === undefined || this._view.buffer !== this._memory.buffer) {
            this._view = new DataView(this._memory.buffer, this._ptr, this._size);
        }
        return this._view;
    }
    getUint8(offset) {
        return this.view.getUint8(offset);
    }
    getInt8(offset) {
        return this.view.getInt8(offset);
    }
    getUint16(offset) {
        this.assertAlignment(offset, Alignment.halfWord);
        return this.view.getUint16(offset, true);
    }
    getInt16(offset) {
        this.assertAlignment(offset, Alignment.halfWord);
        return this.view.getInt16(offset, true);
    }
    getUint32(offset) {
        this.assertAlignment(offset, Alignment.word);
        return this.view.getUint32(offset, true);
    }
    getInt32(offset) {
        this.assertAlignment(offset, Alignment.word);
        return this.view.getInt32(offset, true);
    }
    getUint64(offset) {
        this.assertAlignment(offset, Alignment.doubleWord);
        return this.view.getBigUint64(offset, true);
    }
    getInt64(offset) {
        this.assertAlignment(offset, Alignment.doubleWord);
        return this.view.getBigInt64(offset, true);
    }
    getFloat32(offset) {
        this.assertAlignment(offset, Alignment.word);
        return this.view.getFloat32(offset, true);
    }
    getFloat64(offset) {
        this.assertAlignment(offset, Alignment.doubleWord);
        return this.view.getFloat64(offset, true);
    }
    getPtr(offset) {
        this.assertAlignment(offset, Alignment.word);
        return this.view.getUint32(offset, true);
    }
    getUint8Array(offset, length) {
        return this.getArray(offset, length, Uint8Array);
    }
    getInt8Array(offset, length) {
        return this.getArray(offset, length, Int8Array);
    }
    getUint16Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Uint16Array);
    }
    getInt16Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Int16Array);
    }
    getUint32Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Uint32Array);
    }
    getInt32Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Int32Array);
    }
    getUint64Array(byteOffset, length) {
        return this.getBigArray(byteOffset, length, BigUint64Array);
    }
    getInt64Array(byteOffset, length) {
        return this.getBigArray(byteOffset, length, BigInt64Array);
    }
    getFloat32Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Float32Array);
    }
    getFloat64Array(byteOffset, length) {
        return this.getArray(byteOffset, length, Float64Array);
    }
    copyBytes(offset, length, into, into_offset) {
        if (offset + length > this.size) {
            throw new MemoryError(`Memory access is out of bounds. Accessing [${offset}, ${length}], allocated[${this.ptr}, ${this.size}]`);
        }
        const target = into.getUint8View(into_offset, length);
        target.set(new Uint8Array(this._memory.buffer, this.ptr + offset, length));
    }
    assertAlignment(offset, alignment) {
        if (alignment > this.alignment || offset % alignment !== 0) {
            throw new MemoryError(`Memory location is not aligned to ${alignment}. Allocated[${this.ptr},${this.size}]`);
        }
    }
    getArray(byteOffset, length, clazz) {
        length = length ?? (this.size - byteOffset) / clazz.BYTES_PER_ELEMENT;
        if (!Number.isInteger(length)) {
            throw new MemoryError(`Length must be an integer value. Got ${length}.`);
        }
        const result = new clazz(length);
        result.set(new clazz(this._memory.buffer, this.ptr + byteOffset, length));
        return result;
    }
    getBigArray(byteOffset, length, clazz) {
        length = length ?? (this.size - byteOffset) / clazz.BYTES_PER_ELEMENT;
        if (!Number.isInteger(length)) {
            throw new MemoryError(`Length must be an integer value. Got ${length}.`);
        }
        const result = new clazz(length);
        result.set(new clazz(this._memory.buffer, this.ptr + byteOffset, length));
        return result;
    }
}
exports.BaseMemoryRange = BaseMemoryRange;
class ReadonlyMemoryRange extends BaseMemoryRange {
    constructor(memory, ptr, size) {
        super(memory, ptr, size);
    }
    range(offset, size) {
        if (offset + size > this.size) {
            throw new MemoryError(`Memory access is out of bounds. Accessing [${offset}, ${size}], allocated[${this.ptr}, ${this.size}]`);
        }
        return new ReadonlyMemoryRange(this._memory, this.ptr + offset, size);
    }
}
exports.ReadonlyMemoryRange = ReadonlyMemoryRange;
class MemoryRange extends BaseMemoryRange {
    isAllocated;
    constructor(memory, ptr, size, isPreallocated = false) {
        super(memory, ptr, size);
        this.isAllocated = isPreallocated;
    }
    free() {
        if (typeof this._memory.free !== 'function') {
            throw new MemoryError(`Memory doesn't support free`);
        }
        this._memory.free(this);
    }
    range(offset, size) {
        if (offset + size > this.size) {
            throw new MemoryError(`Memory access is out of bounds. Accessing [${offset}, ${size}], allocated[${this.ptr}, ${this.size}]`);
        }
        return new MemoryRange(this._memory, this.ptr + offset, size);
    }
    setUint8(offset, value) {
        this.view.setUint8(offset, value);
    }
    setInt8(offset, value) {
        this.view.setInt8(offset, value);
    }
    setUint16(offset, value) {
        this.assertAlignment(offset, Alignment.halfWord);
        this.view.setUint16(offset, value, true);
    }
    setInt16(offset, value) {
        this.assertAlignment(offset, Alignment.halfWord);
        this.view.setInt16(offset, value, true);
    }
    setUint32(offset, value) {
        this.assertAlignment(offset, Alignment.word);
        this.view.setUint32(offset, value, true);
    }
    setInt32(offset, value) {
        this.assertAlignment(offset, Alignment.word);
        this.view.setInt32(offset, value, true);
    }
    setUint64(offset, value) {
        this.assertAlignment(offset, Alignment.doubleWord);
        this.view.setBigUint64(offset, value, true);
    }
    setInt64(offset, value) {
        this.assertAlignment(offset, Alignment.doubleWord);
        this.view.setBigInt64(offset, value, true);
    }
    setFloat32(offset, value) {
        this.assertAlignment(offset, Alignment.word);
        this.view.setFloat32(offset, value, true);
    }
    setFloat64(offset, value) {
        this.assertAlignment(offset, Alignment.doubleWord);
        this.view.setFloat64(offset, value, true);
    }
    setPtr(offset, value) {
        this.assertAlignment(offset, Alignment.word);
        this.view.setUint32(offset, value, true);
    }
    getUint8View(offset, length) {
        return this.getArrayView(offset, length, Uint8Array);
    }
    getInt8View(offset, length) {
        return this.getArrayView(offset, length, Int8Array);
    }
    getUint16View(offset, length) {
        return this.getArrayView(offset, length, Uint16Array);
    }
    getInt16View(offset, length) {
        return this.getArrayView(offset, length, Int16Array);
    }
    getUint32View(offset, length) {
        return this.getArrayView(offset, length, Uint32Array);
    }
    getInt32View(offset, length) {
        return this.getArrayView(offset, length, Int32Array);
    }
    getUint64View(offset, length) {
        return this.getBigArrayView(offset, length, BigUint64Array);
    }
    getInt64View(offset, length) {
        return this.getBigArrayView(offset, length, BigInt64Array);
    }
    getFloat32View(offset, length) {
        return this.getArrayView(offset, length, Float32Array);
    }
    getFloat64View(offset, length) {
        return this.getArrayView(offset, length, Float64Array);
    }
    setUint8Array(offset, bytes) {
        this.setArray(offset, bytes, Uint8Array);
    }
    setInt8Array(offset, bytes) {
        this.setArray(offset, bytes, Int8Array);
    }
    setUint16Array(offset, bytes) {
        this.setArray(offset, bytes, Uint16Array);
    }
    setInt16Array(offset, bytes) {
        this.setArray(offset, bytes, Int16Array);
    }
    setUint32Array(offset, bytes) {
        this.setArray(offset, bytes, Uint32Array);
    }
    setInt32Array(offset, bytes) {
        this.setArray(offset, bytes, Int32Array);
    }
    setUint64Array(offset, bytes) {
        this.setBigArray(offset, bytes, BigUint64Array);
    }
    setInt64Array(offset, bytes) {
        this.setBigArray(offset, bytes, BigInt64Array);
    }
    setFloat32Array(offset, bytes) {
        this.setArray(offset, bytes, Float32Array);
    }
    setFloat64Array(offset, bytes) {
        this.setArray(offset, bytes, Float64Array);
    }
    getArrayView(byteOffset, length, clazz) {
        length = length ?? (this.size - byteOffset) / clazz.BYTES_PER_ELEMENT;
        if (!Number.isInteger(length)) {
            throw new MemoryError(`Length must be an integer value. Got ${length}.`);
        }
        return new clazz(this._memory.buffer, this.ptr + byteOffset, length);
    }
    getBigArrayView(byteOffset, length, clazz) {
        length = length ?? (this.size - byteOffset) / clazz.BYTES_PER_ELEMENT;
        if (!Number.isInteger(length)) {
            throw new MemoryError(`Length must be an integer value. Got ${length}.`);
        }
        return new clazz(this._memory.buffer, this.ptr + byteOffset, length);
    }
    setArray(byteOffset, bytes, clazz) {
        new clazz(this._memory.buffer, this.ptr + byteOffset, bytes.length).set(bytes);
    }
    setBigArray(byteOffset, bytes, clazz) {
        new clazz(this._memory.buffer, this.ptr + byteOffset, bytes.length).set(bytes);
    }
}
exports.MemoryRange = MemoryRange;
/**
 * A memory of size 0. Doesn't allow any kind of operation on it.
 */
class NullMemory {
    id = 'b60336d2-c856-4767-af3b-f66e1ab6c507';
    buffer = new ArrayBuffer(0);
    alloc() {
        throw new MemoryError('Cannot allocate memory on a null memory.');
    }
    realloc() {
        throw new MemoryError('Cannot re-allocate memory on a null memory.');
    }
    preAllocated() {
        throw new MemoryError('Cannot point to pre-allocate memory on a null memory.');
    }
    readonly() {
        throw new MemoryError('Cannot point to readonly memory on a null memory.');
    }
    free() {
        throw new MemoryError('Cannot free memory on a null memory.');
    }
}
var Memory;
(function (Memory) {
    Memory.Null = new NullMemory();
    class Default {
        id;
        memory;
        cabi_realloc;
        constructor(exports, id) {
            if (exports.memory === undefined || exports.cabi_realloc === undefined) {
                throw new MemoryError('The exports object must contain a memory object and a cabi_realloc function.');
            }
            this.id = id ?? uuid.v4();
            this.memory = exports.memory;
            this.cabi_realloc = exports.cabi_realloc;
        }
        get buffer() {
            return this.memory.buffer;
        }
        alloc(align, size) {
            const ptr = this.cabi_realloc(0, 0, align, size);
            return new MemoryRange(this, ptr, size);
        }
        realloc(range, newSize) {
            const ptr = this.cabi_realloc(range.ptr, range.size, range.alignment, newSize);
            return new MemoryRange(this, ptr, newSize);
        }
        preAllocated(ptr, size) {
            return new MemoryRange(this, ptr, size);
        }
        readonly(ptr, size) {
            return new ReadonlyMemoryRange(this, ptr, size);
        }
    }
    Memory.Default = Default;
})(Memory || (exports.Memory = Memory = {}));
var Options;
(function (Options) {
    function is(value) {
        const candidate = value;
        return candidate && typeof candidate.encoding === 'string' && (candidate.keepOption === undefined || typeof candidate.keepOption === 'boolean');
    }
    Options.is = is;
})(Options || (exports.Options = Options = {}));
var FlatTypeKind;
(function (FlatTypeKind) {
    FlatTypeKind["i32"] = "i32";
    FlatTypeKind["i64"] = "i64";
    FlatTypeKind["f32"] = "f32";
    FlatTypeKind["f64"] = "f64";
})(FlatTypeKind || (exports.FlatTypeKind = FlatTypeKind = {}));
var $i32;
(function ($i32) {
    $i32.kind = FlatTypeKind.i32;
    $i32.size = 4;
    $i32.alignment = Alignment.word;
    function load(memory, offset) {
        return memory.getUint32(offset);
    }
    $i32.load = load;
    function store(memory, offset, value) {
        memory.setUint32(offset, value);
    }
    $i32.store = store;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, $i32.alignment);
        src.assertAlignment(src_offset, $i32.alignment);
        src.copyBytes(src_offset, $i32.size, dest, dest_offset);
    }
    $i32.copy = copy;
})($i32 || ($i32 = {}));
exports.i32 = $i32;
var $i64;
(function ($i64) {
    $i64.kind = FlatTypeKind.i64;
    $i64.size = 8;
    $i64.alignment = Alignment.doubleWord;
    function load(memory, offset) {
        return memory.getUint64(offset);
    }
    $i64.load = load;
    function store(memory, offset, value) {
        memory.setUint64(offset, value);
    }
    $i64.store = store;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, $i64.alignment);
        src.assertAlignment(src_offset, $i64.alignment);
        src.copyBytes(src_offset, $i64.size, dest, dest_offset);
    }
    $i64.copy = copy;
})($i64 || ($i64 = {}));
exports.i64 = $i64;
var $f32;
(function ($f32) {
    $f32.kind = FlatTypeKind.f32;
    $f32.size = 4;
    $f32.alignment = Alignment.word;
    function load(memory, offset) {
        return memory.getFloat32(offset);
    }
    $f32.load = load;
    function store(memory, offset, value) {
        memory.setFloat32(offset, value);
    }
    $f32.store = store;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, $f32.alignment);
        src.assertAlignment(src_offset, $f32.alignment);
        src.copyBytes(src_offset, $f32.size, dest, dest_offset);
    }
    $f32.copy = copy;
})($f32 || ($f32 = {}));
exports.f32 = $f32;
var $f64;
(function ($f64) {
    $f64.kind = FlatTypeKind.f64;
    $f64.size = 8;
    $f64.alignment = Alignment.doubleWord;
    function load(memory, offset) {
        return memory.getFloat64(offset);
    }
    $f64.load = load;
    function store(memory, offset, value) {
        memory.setFloat64(offset, value);
    }
    $f64.store = store;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, $f64.alignment);
        src.assertAlignment(src_offset, $f64.alignment);
        src.copyBytes(src_offset, $f64.size, dest, dest_offset);
    }
    $f64.copy = copy;
})($f64 || ($f64 = {}));
exports.f64 = $f64;
class FlatTuple {
    types;
    alignment;
    size;
    constructor(types) {
        this.types = types;
        this.alignment = FlatTuple.alignment(types);
        this.size = FlatTuple.size(types, this.alignment);
    }
    load(memory, offset) {
        memory.assertAlignment(offset, this.alignment);
        const result = [];
        for (const type of this.types) {
            offset = align(offset, type.alignment);
            result.push(type.load(memory, offset));
            offset += type.size;
        }
        return result;
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, values) {
        memory.assertAlignment(offset, this.alignment);
        for (const [index, type] of this.types.entries()) {
            const value = values[index];
            offset = align(offset, type.alignment);
            type.store(memory, offset, value);
            offset += type.size;
        }
    }
    copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, this.alignment);
        src.assertAlignment(src_offset, this.alignment);
        src.copyBytes(src_offset, this.size, dest, dest_offset);
    }
    static alignment(types) {
        let result = Alignment.byte;
        for (const type of types) {
            result = Math.max(result, type.alignment);
        }
        return result;
    }
    static size(types, tupleAlignment) {
        let result = 0;
        for (const type of types) {
            result = align(result, type.alignment);
            result += type.size;
        }
        return align(result, tupleAlignment);
    }
}
exports.FlatTuple = FlatTuple;
var WasmTypes;
(function (WasmTypes) {
    const $32 = new DataView(new ArrayBuffer(4));
    const $64 = new DataView(new ArrayBuffer(8));
    function reinterpret_i32_as_f32(i32) {
        $32.setInt32(0, i32, true);
        return $32.getFloat32(0, true);
    }
    WasmTypes.reinterpret_i32_as_f32 = reinterpret_i32_as_f32;
    function reinterpret_f32_as_i32(f32) {
        $32.setFloat32(0, f32, true);
        return $32.getInt32(0, true);
    }
    WasmTypes.reinterpret_f32_as_i32 = reinterpret_f32_as_i32;
    function convert_i64_to_i32(i64) {
        return BigInts.asNumber(i64);
    }
    WasmTypes.convert_i64_to_i32 = convert_i64_to_i32;
    function convert_i32_to_i64(i32) {
        return BigInt(i32);
    }
    WasmTypes.convert_i32_to_i64 = convert_i32_to_i64;
    function reinterpret_i64_as_f32(i64) {
        const i32 = convert_i64_to_i32(i64);
        return reinterpret_i32_as_f32(i32);
    }
    WasmTypes.reinterpret_i64_as_f32 = reinterpret_i64_as_f32;
    function reinterpret_f32_as_i64(f32) {
        const i32 = reinterpret_f32_as_i32(f32);
        return convert_i32_to_i64(i32);
    }
    WasmTypes.reinterpret_f32_as_i64 = reinterpret_f32_as_i64;
    function reinterpret_i64_as_f64(i64) {
        $64.setBigInt64(0, i64, true);
        return $64.getFloat64(0, true);
    }
    WasmTypes.reinterpret_i64_as_f64 = reinterpret_i64_as_f64;
    function reinterpret_f64_as_i64(f64) {
        $64.setFloat64(0, f64, true);
        return $64.getBigInt64(0, true);
    }
    WasmTypes.reinterpret_f64_as_i64 = reinterpret_f64_as_i64;
})(WasmTypes || (WasmTypes = {}));
class CoerceValueIter {
    values;
    haveFlatTypes;
    wantFlatTypes;
    index;
    constructor(values, haveFlatTypes, wantFlatTypes) {
        this.values = values;
        this.haveFlatTypes = haveFlatTypes;
        this.wantFlatTypes = wantFlatTypes;
        if (haveFlatTypes.length < wantFlatTypes.length) {
            throw new ComponentModelTrap(`Invalid coercion: have ${haveFlatTypes.length} values, want ${wantFlatTypes.length} values`);
        }
        this.index = 0;
    }
    next() {
        const value = this.values.next();
        if (value.done) {
            return value;
        }
        const haveType = this.haveFlatTypes[this.index];
        const wantType = this.wantFlatTypes[this.index++];
        if (haveType === $i32 && wantType === $f32) {
            return { done: false, value: WasmTypes.reinterpret_i32_as_f32(value.value) };
        }
        else if (haveType === $i64 && wantType === $i32) {
            return { done: false, value: WasmTypes.convert_i64_to_i32(value.value) };
        }
        else if (haveType === $i64 && wantType === $f32) {
            return { done: false, value: WasmTypes.reinterpret_i64_as_f32(value.value) };
        }
        else if (haveType === $i64 && wantType === $f64) {
            return { done: false, value: WasmTypes.reinterpret_i64_as_f64(value.value) };
        }
        else {
            return value;
        }
    }
}
var ComponentModelTypeKind;
(function (ComponentModelTypeKind) {
    ComponentModelTypeKind["bool"] = "bool";
    ComponentModelTypeKind["u8"] = "u8";
    ComponentModelTypeKind["u16"] = "u16";
    ComponentModelTypeKind["u32"] = "u32";
    ComponentModelTypeKind["u64"] = "u64";
    ComponentModelTypeKind["s8"] = "s8";
    ComponentModelTypeKind["s16"] = "s16";
    ComponentModelTypeKind["s32"] = "s32";
    ComponentModelTypeKind["s64"] = "s64";
    ComponentModelTypeKind["float32"] = "float32";
    ComponentModelTypeKind["float64"] = "float64";
    ComponentModelTypeKind["char"] = "char";
    ComponentModelTypeKind["string"] = "string";
    ComponentModelTypeKind["list"] = "list";
    ComponentModelTypeKind["record"] = "record";
    ComponentModelTypeKind["tuple"] = "tuple";
    ComponentModelTypeKind["variant"] = "variant";
    ComponentModelTypeKind["enum"] = "enum";
    ComponentModelTypeKind["flags"] = "flags";
    ComponentModelTypeKind["option"] = "option";
    ComponentModelTypeKind["result"] = "result";
    ComponentModelTypeKind["resource"] = "resource";
    ComponentModelTypeKind["resourceHandle"] = "resourceHandle";
    ComponentModelTypeKind["borrow"] = "borrow";
    ComponentModelTypeKind["own"] = "own";
})(ComponentModelTypeKind || (exports.ComponentModelTypeKind = ComponentModelTypeKind = {}));
var ComponentModelContext;
(function (ComponentModelContext) {
    function is(value) {
        const candidate = value;
        return candidate && Options.is(candidate.options) && ResourceManagers.is(candidate.resources);
    }
    ComponentModelContext.is = is;
})(ComponentModelContext || (exports.ComponentModelContext = ComponentModelContext = {}));
var ComponentModelType;
(function (ComponentModelType) {
    function satisfies(_value) {
        // This is for pure Type checking.
    }
    ComponentModelType.satisfies = satisfies;
})(ComponentModelType || (ComponentModelType = {}));
class ResultError extends Error {
    cause;
    constructor(message, cause) {
        super(message, { cause: cause });
        this.cause = cause;
    }
}
exports.ResultError = ResultError;
var bool;
(function (bool) {
    bool.kind = ComponentModelTypeKind.bool;
    bool.size = 1;
    bool.alignment = Alignment.byte;
    bool.flatTypes = [$i32];
    function load(memory, offset) {
        return memory.getUint8(offset) !== 0;
    }
    bool.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < 0) {
            throw new ComponentModelTrap(`Invalid bool value ${value}`);
        }
        return value !== 0;
    }
    bool.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(bool.alignment, bool.size);
    }
    bool.alloc = alloc;
    function store(memory, offset, value) {
        memory.setUint8(offset, value ? 1 : 0);
    }
    bool.store = store;
    function lowerFlat(result, _memory, value) {
        result.push(value ? 1 : 0);
    }
    bool.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        src.copyBytes(src_offset, bool.size, dest, dest_offset);
    }
    bool.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    bool.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    bool.Error = Error;
})(bool || (exports.bool = bool = {}));
ComponentModelType.satisfies(bool);
var u8;
(function (u8) {
    u8.kind = ComponentModelTypeKind.u8;
    u8.size = 1;
    u8.alignment = Alignment.byte;
    u8.flatTypes = [$i32];
    u8.LOW_VALUE = 0;
    u8.HIGH_VALUE = 255;
    function load(memory, offset) {
        return memory.getUint8(offset);
    }
    u8.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < u8.LOW_VALUE || value > u8.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u8 value ${value}`);
        }
        return value;
    }
    u8.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(u8.alignment, u8.size);
    }
    u8.alloc = alloc;
    function store(memory, offset, value) {
        memory.setUint8(offset, value);
    }
    u8.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < u8.LOW_VALUE || value > u8.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u8 value ${value}`);
        }
        result.push(value);
    }
    u8.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        src.copyBytes(src_offset, u8.size, dest, dest_offset);
    }
    u8.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    u8.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    u8.Error = Error;
})(u8 || (exports.u8 = u8 = {}));
ComponentModelType.satisfies(u8);
var u16;
(function (u16) {
    u16.kind = ComponentModelTypeKind.u16;
    u16.size = 2;
    u16.alignment = Alignment.halfWord;
    u16.flatTypes = [$i32];
    u16.LOW_VALUE = 0;
    u16.HIGH_VALUE = 65535;
    function load(memory, offset) {
        return memory.getUint16(offset);
    }
    u16.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < u16.LOW_VALUE || value > u16.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u16 value ${value}`);
        }
        return value;
    }
    u16.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(u16.alignment, u16.size);
    }
    u16.alloc = alloc;
    function store(memory, offset, value) {
        memory.setUint16(offset, value);
    }
    u16.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < u16.LOW_VALUE || value > u16.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u16 value ${value}`);
        }
        result.push(value);
    }
    u16.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, u16.alignment);
        src.assertAlignment(src_offset, u16.alignment);
        src.copyBytes(src_offset, u16.size, dest, dest_offset);
    }
    u16.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    u16.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    u16.Error = Error;
})(u16 || (exports.u16 = u16 = {}));
ComponentModelType.satisfies(u16);
var u32;
(function (u32) {
    u32.kind = ComponentModelTypeKind.u32;
    u32.size = 4;
    u32.alignment = Alignment.word;
    u32.flatTypes = [$i32];
    u32.LOW_VALUE = 0;
    u32.HIGH_VALUE = 4294967295; // 2 ^ 32 - 1
    function valid(value) {
        return value >= u32.LOW_VALUE && value <= u32.HIGH_VALUE && Number.isInteger(value);
    }
    u32.valid = valid;
    function load(memory, offset) {
        return memory.getUint32(offset);
    }
    u32.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < u32.LOW_VALUE || value > u32.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u32 value ${value}`);
        }
        return value;
    }
    u32.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(u32.alignment, u32.size);
    }
    u32.alloc = alloc;
    function store(memory, offset, value) {
        memory.setUint32(offset, value);
    }
    u32.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < u32.LOW_VALUE || value > u32.HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid u32 value ${value}`);
        }
        result.push(value);
    }
    u32.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, u32.alignment);
        src.assertAlignment(src_offset, u32.alignment);
        src.copyBytes(src_offset, u32.size, dest, dest_offset);
    }
    u32.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    u32.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    u32.Error = Error;
})(u32 || (exports.u32 = u32 = {}));
ComponentModelType.satisfies(u32);
var u64;
(function (u64) {
    u64.kind = ComponentModelTypeKind.u64;
    u64.size = 8;
    u64.alignment = Alignment.doubleWord;
    u64.flatTypes = [$i64];
    u64.LOW_VALUE = 0n;
    u64.HIGH_VALUE = 18446744073709551615n; // 2 ^ 64 - 1
    function load(memory, offset) {
        return memory.getUint64(offset);
    }
    u64.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < u64.LOW_VALUE) {
            throw new ComponentModelTrap(`Invalid u64 value ${value}`);
        }
        return value;
    }
    u64.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(u64.alignment, u64.size);
    }
    u64.alloc = alloc;
    function store(memory, offset, value) {
        memory.setUint64(offset, value);
    }
    u64.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < u64.LOW_VALUE) {
            throw new ComponentModelTrap(`Invalid u64 value ${value}`);
        }
        result.push(value);
    }
    u64.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, u64.alignment);
        src.assertAlignment(src_offset, u64.alignment);
        src.copyBytes(src_offset, u64.size, dest, dest_offset);
    }
    u64.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    u64.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    u64.Error = Error;
})(u64 || (exports.u64 = u64 = {}));
ComponentModelType.satisfies(u64);
var s8;
(function (s8) {
    s8.kind = ComponentModelTypeKind.s8;
    s8.size = 1;
    s8.alignment = Alignment.byte;
    s8.flatTypes = [$i32];
    const LOW_VALUE = -128;
    const HIGH_VALUE = 127;
    function load(memory, offset) {
        return memory.getInt8(offset);
    }
    s8.load = load;
    function liftFlat(_memory, values) {
        let value = values.next().value;
        if (!Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s8 value ${value}`);
        }
        // All int values in the component model are transferred as unsigned
        // values. So for signed values we need to convert them back.
        if (value > HIGH_VALUE) {
            value = value - 256;
        }
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid s8 value ${value}`);
        }
        return value;
    }
    s8.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(s8.alignment, s8.size);
    }
    s8.alloc = alloc;
    function store(memory, offset, value) {
        memory.setInt8(offset, value);
    }
    s8.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s8 value ${value}`);
        }
        result.push((value < 0) ? (value + 256) : value);
    }
    s8.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, s8.alignment);
        src.assertAlignment(src_offset, s8.alignment);
        src.copyBytes(src_offset, s8.size, dest, dest_offset);
    }
    s8.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    s8.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    s8.Error = Error;
})(s8 || (exports.s8 = s8 = {}));
ComponentModelType.satisfies(s8);
var s16;
(function (s16) {
    s16.kind = ComponentModelTypeKind.s16;
    s16.size = 2;
    s16.alignment = Alignment.halfWord;
    s16.flatTypes = [$i32];
    const LOW_VALUE = -32768; // -2 ^ 15
    const HIGH_VALUE = 32767; // 2 ^ 15 - 1
    function load(memory, offset) {
        return memory.getInt16(offset);
    }
    s16.load = load;
    function liftFlat(_memory, values) {
        let value = values.next().value;
        if (!Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s16 value ${value}`);
        }
        // See comment in s8.liftFlat
        if (value > HIGH_VALUE) {
            value = value - 65536;
        }
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid s16 value ${value}`);
        }
        return value;
    }
    s16.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(s16.alignment, s16.size);
    }
    s16.alloc = alloc;
    function store(memory, offset, value) {
        memory.setInt16(offset, value);
    }
    s16.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s16 value ${value}`);
        }
        result.push((value < 0) ? (value + 65536) : value);
    }
    s16.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, s16.alignment);
        src.assertAlignment(src_offset, s16.alignment);
        src.copyBytes(src_offset, s16.size, dest, dest_offset);
    }
    s16.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    s16.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    s16.Error = Error;
})(s16 || (exports.s16 = s16 = {}));
ComponentModelType.satisfies(s16);
var s32;
(function (s32) {
    s32.kind = ComponentModelTypeKind.s32;
    s32.size = 4;
    s32.alignment = Alignment.word;
    s32.flatTypes = [$i32];
    const LOW_VALUE = -2147483648; // -2 ^ 31
    const HIGH_VALUE = 2147483647; // 2 ^ 31 - 1
    function load(memory, offset) {
        return memory.getInt32(offset);
    }
    s32.load = load;
    function liftFlat(_memory, values) {
        let value = values.next().value;
        if (!Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s32 value ${value}`);
        }
        // See comment in s8.liftFlat
        if (value > HIGH_VALUE) {
            value = value - 4294967296;
        }
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid s32 value ${value}`);
        }
        return value;
    }
    s32.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(s32.alignment, s32.size);
    }
    s32.alloc = alloc;
    function store(memory, offset, value) {
        memory.setInt32(offset, value);
    }
    s32.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE || !Number.isInteger(value)) {
            throw new ComponentModelTrap(`Invalid s32 value ${value}`);
        }
        result.push((value < 0) ? (value + 4294967296) : value);
    }
    s32.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, s32.alignment);
        src.assertAlignment(src_offset, s32.alignment);
        src.copyBytes(src_offset, s32.size, dest, dest_offset);
    }
    s32.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    s32.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    s32.Error = Error;
})(s32 || (exports.s32 = s32 = {}));
ComponentModelType.satisfies(s32);
var s64;
(function (s64) {
    s64.kind = ComponentModelTypeKind.s64;
    s64.size = 8;
    s64.alignment = Alignment.doubleWord;
    s64.flatTypes = [$i64];
    const LOW_VALUE = -9223372036854775808n; // -2 ^ 63
    const HIGH_VALUE = 9223372036854775807n; // 2 ^ 63 - 1
    function load(memory, offset) {
        return memory.getInt64(offset);
    }
    s64.load = load;
    function liftFlat(_memory, values) {
        let value = values.next().value;
        if (typeof value !== 'bigint') {
            throw new ComponentModelTrap(`Invalid s64 value ${value}`);
        }
        // See comment in s8.liftFlat
        if (value > HIGH_VALUE) {
            value = value - 18446744073709551616n;
        }
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid s64 value ${value}`);
        }
        return value;
    }
    s64.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(s64.alignment, s64.size);
    }
    s64.alloc = alloc;
    function store(memory, offset, value) {
        memory.setInt64(offset, value);
    }
    s64.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid s64 value ${value}`);
        }
        result.push((value < 0) ? (value + 18446744073709551616n) : value);
    }
    s64.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, s64.alignment);
        src.assertAlignment(src_offset, s64.alignment);
        src.copyBytes(src_offset, s64.size, dest, dest_offset);
    }
    s64.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    s64.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    s64.Error = Error;
})(s64 || (exports.s64 = s64 = {}));
ComponentModelType.satisfies(s64);
var float32;
(function (float32) {
    float32.kind = ComponentModelTypeKind.float32;
    float32.size = 4;
    float32.alignment = Alignment.word;
    float32.flatTypes = [$f32];
    const LOW_VALUE = -3.4028234663852886e+38;
    const HIGH_VALUE = 3.4028234663852886e+38;
    const NAN = 0x7fc00000;
    function load(memory, offset) {
        return memory.getFloat32(offset);
    }
    float32.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid float32 value ${value}`);
        }
        return value === NAN ? Number.NaN : value;
    }
    float32.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(float32.alignment, float32.size);
    }
    float32.alloc = alloc;
    function store(memory, offset, value) {
        memory.setFloat32(offset, value);
    }
    float32.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid float32 value ${value}`);
        }
        result.push(Number.isNaN(value) ? NAN : value);
    }
    float32.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, float32.alignment);
        src.assertAlignment(src_offset, float32.alignment);
        src.copyBytes(src_offset, float32.size, dest, dest_offset);
    }
    float32.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    float32.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    float32.Error = Error;
})(float32 || (exports.float32 = float32 = {}));
ComponentModelType.satisfies(float32);
var float64;
(function (float64) {
    float64.kind = ComponentModelTypeKind.float64;
    float64.size = 8;
    float64.alignment = Alignment.doubleWord;
    float64.flatTypes = [$f64];
    const LOW_VALUE = -1 * Number.MAX_VALUE;
    const HIGH_VALUE = Number.MAX_VALUE;
    const NAN = 0x7ff8000000000000;
    function load(memory, offset) {
        return memory.getFloat64(offset);
    }
    float64.load = load;
    function liftFlat(_memory, values) {
        const value = values.next().value;
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid float64 value ${value}`);
        }
        return value === NAN ? Number.NaN : value;
    }
    float64.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(float64.alignment, float64.size);
    }
    float64.alloc = alloc;
    function store(memory, offset, value) {
        memory.setFloat64(offset, value);
    }
    float64.store = store;
    function lowerFlat(result, _memory, value) {
        if (value < LOW_VALUE || value > HIGH_VALUE) {
            throw new ComponentModelTrap(`Invalid float64 value ${value}`);
        }
        result.push(Number.isNaN(value) ? NAN : value);
    }
    float64.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, float64.alignment);
        src.assertAlignment(src_offset, float64.alignment);
        src.copyBytes(src_offset, float64.size, dest, dest_offset);
    }
    float64.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    float64.copyFlat = copyFlat;
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${cause}`, cause);
        }
    }
    float64.Error = Error;
})(float64 || (exports.float64 = float64 = {}));
ComponentModelType.satisfies(float64);
exports.byte = {
    kind: u8.kind,
    size: u8.size,
    alignment: u8.alignment,
    flatTypes: u8.flatTypes,
    load: u8.load,
    liftFlat: u8.liftFlat,
    alloc: u8.alloc,
    store: u8.store,
    lowerFlat: u8.lowerFlat,
    copy: u8.copy,
    copyFlat: u8.copyFlat
};
exports.size = {
    kind: u32.kind,
    size: u32.size,
    alignment: u32.alignment,
    flatTypes: u32.flatTypes,
    load: u32.load,
    liftFlat: u32.liftFlat,
    alloc: u32.alloc,
    store: u32.store,
    lowerFlat: u32.lowerFlat,
    copy: u32.copy,
    copyFlat: u32.copyFlat
};
exports.ptr = {
    kind: u32.kind,
    size: u32.size,
    alignment: u32.alignment,
    flatTypes: u32.flatTypes,
    load: u32.load,
    liftFlat: u32.liftFlat,
    alloc: u32.alloc,
    store: u32.store,
    lowerFlat: u32.lowerFlat,
    copy: u32.copy,
    copyFlat: u32.copyFlat
};
var char;
(function (char) {
    char.kind = ComponentModelTypeKind.char;
    char.size = 4;
    char.alignment = Alignment.word;
    char.flatTypes = [$i32];
    function load(memory, offset, _context) {
        return fromCodePoint(u32.load(memory, offset));
    }
    char.load = load;
    function liftFlat(memory, values, _context) {
        return fromCodePoint(u32.liftFlat(memory, values));
    }
    char.liftFlat = liftFlat;
    function alloc(memory) {
        return u32.alloc(memory);
    }
    char.alloc = alloc;
    function store(memory, offset, value, _context) {
        u32.store(memory, offset, asCodePoint(value));
    }
    char.store = store;
    function lowerFlat(result, memory, value, _context) {
        u32.lowerFlat(result, memory, asCodePoint(value));
    }
    char.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, char.alignment);
        src.assertAlignment(src_offset, char.alignment);
        src.copyBytes(src_offset, char.size, dest, dest_offset);
    }
    char.copy = copy;
    function copyFlat(result, _dest, values, _src) {
        result.push(values.next().value);
    }
    char.copyFlat = copyFlat;
    function fromCodePoint(code) {
        if (code >= 0x110000 || (0xD800 <= code && code <= 0xDFFF)) {
            throw new ComponentModelTrap('Invalid code point');
        }
        return String.fromCodePoint(code);
    }
    function asCodePoint(str) {
        if (str.length !== 1) {
            throw new ComponentModelTrap('String length must be 1');
        }
        const code = str.codePointAt(0);
        if (!(code <= 0xD7FF || (0xD800 <= code && code <= 0x10FFFF))) {
            throw new ComponentModelTrap('Invalid code point');
        }
        return code;
    }
    class Error extends ResultError {
        constructor(value) {
            super(value, `Error value: ${value}`);
        }
    }
    char.Error = Error;
})(char || (exports.char = char = {}));
ComponentModelType.satisfies(char);
var wstring;
(function (wstring) {
    const offsets = {
        data: 0,
        codeUnits: 4
    };
    wstring.kind = ComponentModelTypeKind.string;
    wstring.size = 8;
    wstring.alignment = Alignment.word;
    wstring.flatTypes = [$i32, $i32];
    function load(memRange, offset, context) {
        const dataPtr = memRange.getUint32(offset + offsets.data);
        const codeUnits = memRange.getUint32(offset + offsets.codeUnits);
        return loadFromRange(memRange.memory, dataPtr, codeUnits, context.options);
    }
    wstring.load = load;
    function liftFlat(memory, values, context) {
        const dataPtr = values.next().value;
        const codeUnits = values.next().value;
        return loadFromRange(memory, dataPtr, codeUnits, context.options);
    }
    wstring.liftFlat = liftFlat;
    function alloc(memory) {
        return memory.alloc(wstring.alignment, wstring.size);
    }
    wstring.alloc = alloc;
    function store(memory, offset, str, context) {
        const [ptr, codeUnits] = storeIntoRange(memory.memory, str, context.options);
        memory.setUint32(offset + offsets.data, ptr);
        memory.setUint32(offset + offsets.codeUnits, codeUnits);
    }
    wstring.store = store;
    function lowerFlat(result, memory, str, context) {
        result.push(...storeIntoRange(memory, str, context.options));
    }
    wstring.lowerFlat = lowerFlat;
    function copy(dest, dest_offset, src, src_offset, context) {
        dest.assertAlignment(dest_offset, wstring.alignment);
        src.assertAlignment(src_offset, wstring.alignment);
        // Copy the actual string data
        const data = src.getUint32(src_offset + offsets.data);
        const codeUnits = src.getUint32(src_offset + offsets.codeUnits);
        const [alignment, byteLength] = getAlignmentAndByteLength(codeUnits, context.options);
        const srcReader = src.memory.readonly(data, byteLength);
        const destWriter = dest.memory.alloc(alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
        // Store the new data pointer and code units
        dest.setUint32(dest_offset + offsets.data, destWriter.ptr);
        dest.setUint32(dest_offset + offsets.codeUnits, codeUnits);
    }
    wstring.copy = copy;
    function copyFlat(result, dest, values, src, context) {
        const data = values.next().value;
        const codeUnits = values.next().value;
        // Copy the actual string data
        const [alignment, byteLength] = getAlignmentAndByteLength(codeUnits, context.options);
        const srcReader = src.readonly(data, byteLength);
        const destWriter = dest.alloc(alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
        // Push new ptr and codeUnits
        result.push(destWriter.ptr, codeUnits);
    }
    wstring.copyFlat = copyFlat;
    function getAlignmentAndByteLength(codeUnits, options) {
        const encoding = options.encoding;
        if (encoding === 'latin1+utf-16') {
            throw new ComponentModelTrap('latin1+utf-16 encoding not yet supported');
        }
        if (encoding === 'utf-8') {
            return [u8.alignment, codeUnits];
        }
        else if (encoding === 'utf-16') {
            return [u16.alignment, codeUnits * 2];
        }
        else {
            throw new ComponentModelTrap('Unsupported encoding');
        }
    }
    wstring.getAlignmentAndByteLength = getAlignmentAndByteLength;
    function loadFromRange(memory, data, codeUnits, options) {
        const encoding = options.encoding;
        if (encoding === 'latin1+utf-16') {
            throw new ComponentModelTrap('latin1+utf-16 encoding not yet supported');
        }
        if (encoding === 'utf-8') {
            const byteLength = codeUnits;
            const reader = memory.readonly(data, byteLength);
            return utf8Decoder.decode(reader.getUint8Array(0, byteLength));
        }
        else if (encoding === 'utf-16') {
            const reader = memory.readonly(data, codeUnits * 2);
            return String.fromCharCode(...reader.getUint16Array(data, codeUnits));
        }
        else {
            throw new ComponentModelTrap('Unsupported encoding');
        }
    }
    function storeIntoRange(memory, str, options) {
        const { encoding } = options;
        if (encoding === 'latin1+utf-16') {
            throw new ComponentModelTrap('latin1+utf-16 encoding not yet supported');
        }
        if (encoding === 'utf-8') {
            const data = utf8Encoder.encode(str);
            const writer = memory.alloc(u8.alignment, data.length);
            writer.setUint8Array(0, data);
            return [writer.ptr, data.length];
        }
        else if (encoding === 'utf-16') {
            const writer = memory.alloc(u16.alignment, str.length * 2);
            const data = writer.getUint16View(0);
            for (let i = 0; i < str.length; i++) {
                data[i] = str.charCodeAt(i);
            }
            return [writer.ptr, data.length];
        }
        else {
            throw new ComponentModelTrap('Unsupported encoding');
        }
    }
    class Error extends ResultError {
        constructor(value) {
            super(value, `Error value: ${value}`);
        }
    }
    wstring.Error = Error;
})(wstring || (exports.wstring = wstring = {}));
ComponentModelType.satisfies(wstring);
class ListType {
    static offsets = {
        data: 0,
        length: 4
    };
    elementType;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(elementType) {
        this.elementType = elementType;
        this.kind = ComponentModelTypeKind.list;
        this.size = 8;
        this.alignment = Alignment.word;
        this.flatTypes = [$i32, $i32];
    }
    load(memRange, offset, context) {
        const offsets = ListType.offsets;
        const dataPtr = memRange.getUint32(offset + offsets.data);
        const length = memRange.getUint32(offset + offsets.length);
        return this.loadFromRange(memRange.memory.readonly(dataPtr, length * this.elementType.size), length, context);
    }
    liftFlat(memory, values, context) {
        const dataPtr = values.next().value;
        const length = values.next().value;
        return this.loadFromRange(memory.readonly(dataPtr, length * this.elementType.size), length, context);
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memRange, offset, values, context) {
        const elementMemory = memRange.memory.alloc(this.elementType.alignment, this.elementType.size * values.length);
        this.storeIntoRange(elementMemory, values, context);
        const offsets = ListType.offsets;
        memRange.setUint32(offset + offsets.data, elementMemory.ptr);
        memRange.setUint32(offset + offsets.length, values.length);
    }
    lowerFlat(result, memory, values, context) {
        const elementMemory = memory.alloc(this.elementType.alignment, this.elementType.size * values.length);
        this.storeIntoRange(elementMemory, values, context);
        result.push(elementMemory.ptr, values.length);
    }
    copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, this.alignment);
        src.assertAlignment(src_offset, this.alignment);
        const offsets = ListType.offsets;
        const data = src.getUint32(src_offset + offsets.data);
        const length = src.getUint32(src_offset + offsets.length);
        const byteLength = length * this.elementType.size;
        const srcReader = src.memory.readonly(data, byteLength);
        const destWriter = dest.memory.alloc(this.elementType.alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
        dest.setUint32(dest_offset + offsets.data, destWriter.ptr);
        dest.setUint32(dest_offset + offsets.length, length);
    }
    copyFlat(result, dest, values, src, _context) {
        const data = values.next().value;
        const length = values.next().value;
        const byteLength = length * this.elementType.size;
        const srcReader = src.readonly(data, byteLength);
        const destWriter = dest.alloc(this.elementType.alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
        result.push(destWriter.ptr, length);
    }
    loadFromRange(memory, length, context) {
        const result = [];
        let offset = 0;
        for (let i = 0; i < length; i++) {
            result.push(this.elementType.load(memory, offset, context));
            offset += this.elementType.size;
        }
        return result;
    }
    storeIntoRange(memory, values, context) {
        let offset = 0;
        for (const item of values) {
            this.elementType.store(memory, offset, item, context);
            offset += this.elementType.size;
        }
    }
}
exports.ListType = ListType;
var list;
(function (list) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    list.Error = Error;
})(list || (exports.list = list = {}));
class TypeArrayType {
    static offsets = {
        data: 0,
        length: 4
    };
    kind;
    size;
    alignment;
    flatTypes;
    elementType;
    constructor(elementType) {
        this.kind = ComponentModelTypeKind.list;
        this.size = 8;
        this.alignment = 4;
        this.flatTypes = [$i32, $i32];
        this.elementType = elementType;
    }
    load(memRange, offset) {
        const offsets = TypeArrayType.offsets;
        const dataPtr = memRange.getUint32(offset + offsets.data);
        const length = memRange.getUint32(offset + offsets.length);
        return this.loadFromRange(memRange.memory.readonly(dataPtr, length * this.elementType.size), length);
    }
    liftFlat(memory, values) {
        const dataPtr = values.next().value;
        const length = values.next().value;
        return this.loadFromRange(memory.readonly(dataPtr, length * this.elementType.size), length);
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memRange, offset, value) {
        const writer = memRange.memory.alloc(this.elementType.alignment, value.byteLength);
        this.storeIntoRange(writer, value);
        const offsets = TypeArrayType.offsets;
        memRange.setUint32(offset + offsets.data, writer.ptr);
        memRange.setUint32(offset + offsets.length, value.length);
    }
    lowerFlat(result, memory, value) {
        const writer = memory.alloc(this.elementType.alignment, value.byteLength);
        this.storeIntoRange(writer, value);
        result.push(writer.ptr, value.length);
    }
    copy(dest, dest_offset, src, src_offset) {
        dest.assertAlignment(dest_offset, this.alignment);
        src.assertAlignment(src_offset, this.alignment);
        const offsets = TypeArrayType.offsets;
        src.copyBytes(src_offset, this.size, dest, dest_offset);
        const data = src.getUint32(src_offset + offsets.data);
        const byteLength = src.getUint32(src_offset + offsets.length) * this.elementType.size;
        const srcReader = src.memory.readonly(data, byteLength);
        const destWriter = dest.memory.alloc(this.elementType.alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
    }
    copyFlat(result, dest, values, src, _context) {
        const data = values.next().value;
        const length = values.next().value;
        const byteLength = length * this.elementType.size;
        const srcReader = src.readonly(data, byteLength);
        const destWriter = dest.alloc(this.elementType.alignment, byteLength);
        srcReader.copyBytes(0, byteLength, destWriter, 0);
        result.push(destWriter.ptr, length);
    }
}
class Int8ArrayType extends TypeArrayType {
    constructor() {
        super(s8);
    }
    loadFromRange(memory, length) {
        return memory.getInt8Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setInt8Array(0, value);
    }
}
exports.Int8ArrayType = Int8ArrayType;
(function (Int8ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Int8ArrayType.Error = Error;
})(Int8ArrayType || (exports.Int8ArrayType = Int8ArrayType = {}));
class Int16ArrayType extends TypeArrayType {
    constructor() {
        super(s16);
    }
    loadFromRange(memory, length) {
        return memory.getInt16Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setInt16Array(0, value);
    }
}
exports.Int16ArrayType = Int16ArrayType;
(function (Int16ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Int16ArrayType.Error = Error;
})(Int16ArrayType || (exports.Int16ArrayType = Int16ArrayType = {}));
class Int32ArrayType extends TypeArrayType {
    constructor() {
        super(s32);
    }
    loadFromRange(memory, length) {
        return memory.getInt32Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setInt32Array(0, value);
    }
}
exports.Int32ArrayType = Int32ArrayType;
(function (Int32ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Int32ArrayType.Error = Error;
})(Int32ArrayType || (exports.Int32ArrayType = Int32ArrayType = {}));
class BigInt64ArrayType extends TypeArrayType {
    constructor() {
        super(s64);
    }
    loadFromRange(memory, length) {
        return memory.getInt64Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setInt64Array(0, value);
    }
}
exports.BigInt64ArrayType = BigInt64ArrayType;
(function (BigInt64ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    BigInt64ArrayType.Error = Error;
})(BigInt64ArrayType || (exports.BigInt64ArrayType = BigInt64ArrayType = {}));
class Uint8ArrayType extends TypeArrayType {
    constructor() {
        super(u8);
    }
    loadFromRange(memory, length) {
        return memory.getUint8Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setUint8Array(0, value);
    }
}
exports.Uint8ArrayType = Uint8ArrayType;
(function (Uint8ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Uint8ArrayType.Error = Error;
})(Uint8ArrayType || (exports.Uint8ArrayType = Uint8ArrayType = {}));
class Uint16ArrayType extends TypeArrayType {
    constructor() {
        super(u16);
    }
    loadFromRange(memory, length) {
        return memory.getUint16Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setUint16Array(0, value);
    }
}
exports.Uint16ArrayType = Uint16ArrayType;
(function (Uint16ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Uint16ArrayType.Error = Error;
})(Uint16ArrayType || (exports.Uint16ArrayType = Uint16ArrayType = {}));
class Uint32ArrayType extends TypeArrayType {
    constructor() {
        super(u32);
    }
    loadFromRange(memory, length) {
        return memory.getUint32Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setUint32Array(0, value);
    }
}
exports.Uint32ArrayType = Uint32ArrayType;
(function (Uint32ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Uint32ArrayType.Error = Error;
})(Uint32ArrayType || (exports.Uint32ArrayType = Uint32ArrayType = {}));
class BigUint64ArrayType extends TypeArrayType {
    constructor() {
        super(u64);
    }
    loadFromRange(memory, length) {
        return memory.getUint64Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setUint64Array(0, value);
    }
}
exports.BigUint64ArrayType = BigUint64ArrayType;
(function (BigUint64ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    BigUint64ArrayType.Error = Error;
})(BigUint64ArrayType || (exports.BigUint64ArrayType = BigUint64ArrayType = {}));
class Float32ArrayType extends TypeArrayType {
    constructor() {
        super(float32);
    }
    loadFromRange(memory, length) {
        return memory.getFloat32Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setFloat32Array(0, value);
    }
}
exports.Float32ArrayType = Float32ArrayType;
(function (Float32ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Float32ArrayType.Error = Error;
})(Float32ArrayType || (exports.Float32ArrayType = Float32ArrayType = {}));
class Float64ArrayType extends TypeArrayType {
    constructor() {
        super(float64);
    }
    loadFromRange(memory, length) {
        return memory.getFloat64Array(0, length);
    }
    storeIntoRange(memory, value) {
        memory.setFloat64Array(0, value);
    }
}
exports.Float64ArrayType = Float64ArrayType;
(function (Float64ArrayType) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    Float64ArrayType.Error = Error;
})(Float64ArrayType || (exports.Float64ArrayType = Float64ArrayType = {}));
class BaseRecordType {
    fields;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(fields, kind) {
        this.fields = fields;
        this.kind = kind;
        this.alignment = BaseRecordType.alignment(fields);
        this.size = BaseRecordType.size(fields, this.alignment);
        this.flatTypes = BaseRecordType.flatTypes(fields);
    }
    load(memory, offset, context) {
        memory.assertAlignment(offset, this.alignment);
        const result = [];
        for (const field of this.fields) {
            offset = align(offset, field.type.alignment);
            result.push(field.type.load(memory, offset, context));
            offset += field.type.size;
        }
        return this.create(this.fields, result);
    }
    liftFlat(memory, values, context) {
        const result = [];
        for (const field of this.fields) {
            result.push(field.type.liftFlat(memory, values, context));
        }
        return this.create(this.fields, result);
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, record, context) {
        memory.assertAlignment(offset, this.alignment);
        const values = this.elements(record, this.fields);
        for (let i = 0; i < this.fields.length; i++) {
            const field = this.fields[i];
            const value = values[i];
            offset = align(offset, field.type.alignment);
            field.type.store(memory, offset, value, context);
            offset += field.type.size;
        }
    }
    lowerFlat(result, memory, record, context) {
        const values = this.elements(record, this.fields);
        for (let i = 0; i < this.fields.length; i++) {
            const field = this.fields[i];
            const value = values[i];
            field.type.lowerFlat(result, memory, value, context);
        }
    }
    copy(dest, dest_offset, src, src_offset, context) {
        for (const field of this.fields) {
            dest_offset = align(dest_offset, field.type.alignment);
            src_offset = align(src_offset, field.type.alignment);
            field.type.copy(dest, dest_offset, src, src_offset, context);
            dest_offset += field.type.size;
            src_offset += field.type.size;
        }
    }
    copyFlat(result, dest, values, src, context) {
        for (const field of this.fields) {
            field.type.copyFlat(result, dest, values, src, context);
        }
    }
    static size(fields, recordAlignment) {
        let result = 0;
        for (const field of fields) {
            result = align(result, field.type.alignment);
            result += field.type.size;
        }
        return align(result, recordAlignment);
    }
    static alignment(fields) {
        let result = 1;
        for (const field of fields) {
            result = Math.max(result, field.type.alignment);
        }
        return result;
    }
    static flatTypes(fields) {
        const result = [];
        for (const field of fields) {
            result.push(...field.type.flatTypes);
        }
        return result;
    }
}
var RecordField;
(function (RecordField) {
    function create(name, type) {
        return { name, type };
    }
    RecordField.create = create;
})(RecordField || (RecordField = {}));
class RecordType extends BaseRecordType {
    constructor(fields) {
        const recordFields = [];
        for (const [name, type] of fields) {
            recordFields.push(RecordField.create(name, type));
        }
        super(recordFields, ComponentModelTypeKind.record);
    }
    create(fields, values) {
        const result = {};
        for (let i = 0; i < fields.length; i++) {
            const field = fields[i];
            const value = values[i];
            result[field.name] = value;
        }
        return result;
    }
    elements(record, fields) {
        const result = [];
        for (const field of fields) {
            const value = record[field.name];
            result.push(value);
        }
        return result;
    }
}
exports.RecordType = RecordType;
var TupleField;
(function (TupleField) {
    function create(type) {
        return { type };
    }
    TupleField.create = create;
})(TupleField || (TupleField = {}));
class TupleType extends BaseRecordType {
    constructor(fields) {
        const tupleFields = [];
        for (const type of fields) {
            tupleFields.push(TupleField.create(type));
        }
        super(tupleFields, ComponentModelTypeKind.tuple);
    }
    create(_fields, values) {
        return values;
    }
    elements(record, _fields) {
        return record;
    }
}
exports.TupleType = TupleType;
var tuple;
(function (tuple) {
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    tuple.Error = Error;
})(tuple || (exports.tuple = tuple = {}));
class FlagsType {
    type;
    arraySize;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(numberOfFlags) {
        this.kind = ComponentModelTypeKind.flags;
        this.size = FlagsType.size(numberOfFlags);
        this.alignment = FlagsType.alignment(numberOfFlags);
        this.flatTypes = FlagsType.flatTypes(numberOfFlags);
        this.type = FlagsType.getType(numberOfFlags);
        this.arraySize = FlagsType.num32Flags(numberOfFlags);
    }
    load(memory, offset, context) {
        return this.type === undefined ? 0 : this.loadFrom(this.type.load(memory, offset, context));
    }
    liftFlat(memory, values, context) {
        return this.type === undefined ? 0 : this.loadFrom(this.type.liftFlat(memory, values, context));
    }
    loadFrom(value) {
        if (typeof value === 'number') {
            return value;
        }
        else {
            let result = 0n;
            for (let f = 0, i = value.length - 1; f < value.length; f++, i--) {
                const bits = value[i];
                result = result | (BigInt(bits) << BigInt(f * 32));
            }
            return result;
        }
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, flags, context) {
        if (this.type !== undefined) {
            this.type.store(memory, offset, this.storeInto(flags), context);
        }
    }
    lowerFlat(result, _memory, flags, context) {
        if (this.type !== undefined) {
            this.type.lowerFlat(result, _memory, this.storeInto(flags), context);
        }
    }
    copy(dest, dest_offset, src, src_offset, context) {
        if (this.type !== undefined) {
            this.type.copy(dest, dest_offset, src, src_offset, context);
        }
    }
    copyFlat(result, dest, values, src, context) {
        if (this.type !== undefined) {
            this.type.copyFlat(result, dest, values, src, context);
        }
    }
    storeInto(value) {
        if (typeof value === 'number') {
            return value;
        }
        else {
            const result = new Array(this.arraySize).fill(0);
            for (let f = 0, i = result.length - 1; f < result.length; f++, i--) {
                const bits = Number((value >> BigInt(f * 32)) & BigInt(0xffffffff));
                result[i] = bits;
            }
            return result;
        }
    }
    static size(numberOfFlags) {
        if (numberOfFlags === 0) {
            return 0;
        }
        else if (numberOfFlags <= 8) {
            return 1;
        }
        else if (numberOfFlags <= 16) {
            return 2;
        }
        else {
            return 4 * this.num32Flags(numberOfFlags);
        }
    }
    static alignment(numberOfFlags) {
        if (numberOfFlags <= 8) {
            return 1;
        }
        else if (numberOfFlags <= 16) {
            return 2;
        }
        else {
            return 4;
        }
    }
    static getType(numberOfFlags) {
        if (numberOfFlags === 0) {
            return undefined;
        }
        else if (numberOfFlags <= 8) {
            return u8;
        }
        else if (numberOfFlags <= 16) {
            return u16;
        }
        else if (numberOfFlags <= 32) {
            return u32;
        }
        else {
            return new TupleType(new Array(this.num32Flags(numberOfFlags)).fill(u32));
        }
    }
    static flatTypes(numberOfFlags) {
        return new Array(this.num32Flags(numberOfFlags)).fill($i32);
    }
    static num32Flags(numberOfFlags) {
        return Math.ceil(numberOfFlags / 32);
    }
}
exports.FlagsType = FlagsType;
var VariantCase;
(function (VariantCase) {
    function create(index, tag, type) {
        return { index, tag, type, wantFlatTypes: type !== undefined ? [] : undefined };
    }
    VariantCase.create = create;
})(VariantCase || (VariantCase = {}));
class VariantType {
    cases;
    case2Index;
    ctor;
    discriminantType;
    maxCaseAlignment;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(variants, ctor, kind = ComponentModelTypeKind.variant) {
        const cases = [];
        this.case2Index = new Map();
        for (let i = 0; i < variants.length; i++) {
            const type = variants[i][1];
            const name = variants[i][0];
            this.case2Index.set(name, i);
            cases.push(VariantCase.create(i, name, type));
        }
        this.cases = cases;
        this.ctor = ctor;
        this.discriminantType = VariantType.discriminantType(cases.length);
        this.maxCaseAlignment = VariantType.maxCaseAlignment(cases);
        this.kind = kind;
        this.size = VariantType.size(this.discriminantType, cases);
        this.alignment = VariantType.alignment(this.discriminantType, cases);
        this.flatTypes = VariantType.flatTypes(this.discriminantType, cases);
    }
    load(memory, offset, context) {
        const caseIndex = this.discriminantType.load(memory, offset);
        const caseVariant = this.cases[caseIndex];
        if (caseVariant.type === undefined) {
            return this.ctor(caseVariant.tag, undefined);
        }
        else {
            offset += this.discriminantType.size;
            offset = align(offset, this.maxCaseAlignment);
            const value = caseVariant.type.load(memory, offset, context);
            return this.ctor(caseVariant.tag, value);
        }
    }
    liftFlat(memory, values, context) {
        // First one is the discriminant type. So skip it.
        let valuesToReadOver = this.flatTypes.length - 1;
        const caseIndex = this.discriminantType.liftFlat(memory, values);
        const caseVariant = this.cases[caseIndex];
        let result;
        if (caseVariant.type === undefined) {
            result = this.ctor(caseVariant.tag, undefined);
        }
        else {
            // The first flat type is the discriminant type. So skip it.
            const wantFlatTypes = caseVariant.wantFlatTypes;
            const iter = new CoerceValueIter(values, this.flatTypes.slice(1), wantFlatTypes);
            const value = caseVariant.type.liftFlat(memory, iter, context);
            result = this.ctor(caseVariant.tag, value);
            valuesToReadOver = valuesToReadOver - wantFlatTypes.length;
        }
        for (let i = 0; i < valuesToReadOver; i++) {
            values.next();
        }
        return result;
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, variantValue, context) {
        const index = this.case2Index.get(variantValue.tag);
        if (index === undefined) {
            throw new ComponentModelTrap(`Variant case ${variantValue.tag} not found`);
        }
        this.discriminantType.store(memory, offset, index);
        offset += this.discriminantType.size;
        const c = this.cases[index];
        if (c.type !== undefined && variantValue.value !== undefined) {
            offset = align(offset, this.maxCaseAlignment);
            c.type.store(memory, offset, variantValue.value, context);
        }
    }
    lowerFlat(result, memory, variantValue, context) {
        const flatTypes = this.flatTypes;
        const index = this.case2Index.get(variantValue.tag);
        if (index === undefined) {
            throw new ComponentModelTrap(`Variant case ${variantValue.tag} not found`);
        }
        this.discriminantType.lowerFlat(result, memory, index);
        const c = this.cases[index];
        // First one is the discriminant type. So skip it.
        let valuesToFill = this.flatTypes.length - 1;
        if (c.type !== undefined && variantValue.value !== undefined) {
            const payload = [];
            c.type.lowerFlat(payload, memory, variantValue.value, context);
            // First one is the discriminant type. So skip it.
            const wantTypes = flatTypes.slice(1);
            const haveTypes = c.wantFlatTypes;
            if (payload.length !== haveTypes.length) {
                throw new ComponentModelTrap('Mismatched flat types');
            }
            for (let i = 0; i < wantTypes.length; i++) {
                const have = haveTypes[i];
                const want = wantTypes[i];
                if (have === $f32 && want === $i32) {
                    payload[i] = WasmTypes.reinterpret_f32_as_i32(payload[i]);
                }
                else if (have === $i32 && want === $i64) {
                    payload[i] = WasmTypes.convert_i32_to_i64(payload[i]);
                }
                else if (have === $f32 && want === $i64) {
                    payload[i] = WasmTypes.reinterpret_f32_as_i64(payload[i]);
                }
                else if (have === $f64 && want === $i64) {
                    payload[i] = WasmTypes.reinterpret_f64_as_i64(payload[i]);
                }
            }
            valuesToFill = valuesToFill - payload.length;
            result.push(...payload);
        }
        for (let i = flatTypes.length - valuesToFill; i < flatTypes.length; i++) {
            const type = flatTypes[i];
            if (type === $i64) {
                result.push(0n);
            }
            else {
                result.push(0);
            }
        }
    }
    copy(dest, dest_offset, src, src_offset, context) {
        this.discriminantType.copy(dest, dest_offset, src, src_offset);
        const caseIndex = this.discriminantType.load(src, src_offset);
        const caseVariant = this.cases[caseIndex];
        if (caseVariant.type === undefined) {
            return;
        }
        src_offset += this.discriminantType.size;
        src_offset = align(src_offset, this.maxCaseAlignment);
        dest_offset += this.discriminantType.size;
        dest_offset = align(dest_offset, this.maxCaseAlignment);
        caseVariant.type.copy(dest, dest_offset, src, src_offset, context);
    }
    copyFlat(result, dest, values, src, context) {
        let valuesToCopy = this.flatTypes.length - 1;
        this.discriminantType.copyFlat(result, dest, values, src);
        const caseIndex = result[result.length - 1];
        const caseVariant = this.cases[caseIndex];
        if (caseVariant.type !== undefined) {
            const wantFlatTypes = caseVariant.wantFlatTypes;
            // The first flat type is the discriminant type. So skip it.
            const iter = new CoerceValueIter(values, this.flatTypes.slice(1), wantFlatTypes);
            caseVariant.type.copyFlat(result, dest, iter, src, context);
            valuesToCopy = valuesToCopy - wantFlatTypes.length;
        }
        for (let i = 0; i < valuesToCopy; i++) {
            result.push(values.next().value);
        }
    }
    static size(discriminantType, cases) {
        let result = discriminantType.size;
        result = align(result, this.maxCaseAlignment(cases));
        return result + this.maxCaseSize(cases);
    }
    static alignment(discriminantType, cases) {
        return Math.max(discriminantType.alignment, this.maxCaseAlignment(cases));
    }
    static flatTypes(discriminantType, cases) {
        const flat = [];
        for (const c of cases) {
            if (c.type === undefined) {
                continue;
            }
            const flatTypes = c.type.flatTypes;
            for (let i = 0; i < flatTypes.length; i++) {
                const want = flatTypes[i];
                if (i < flat.length) {
                    const use = this.joinFlatType(flat[i], want);
                    flat[i] = use;
                    c.wantFlatTypes.push(want);
                }
                else {
                    flat.push(want);
                    c.wantFlatTypes.push(want);
                }
            }
        }
        return [...discriminantType.flatTypes, ...flat];
    }
    static discriminantType(cases) {
        switch (Math.ceil(Math.log2(cases) / 8)) {
            case 0: return u8;
            case 1: return u8;
            case 2: return u16;
            case 3: return u32;
        }
        throw new ComponentModelTrap(`Too many cases: ${cases}`);
    }
    static maxCaseAlignment(cases) {
        let result = 1;
        for (const c of cases) {
            if (c.type !== undefined) {
                result = Math.max(result, c.type.alignment);
            }
        }
        return result;
    }
    static maxCaseSize(cases) {
        let result = 0;
        for (const c of cases) {
            if (c.type !== undefined) {
                result = Math.max(result, c.type.size);
            }
        }
        return result;
    }
    static joinFlatType(a, b) {
        if (a === b) {
            return a;
        }
        if ((a === $i32 && b === $f32) || (a === $f32 && b === $i32)) {
            return $i32;
        }
        return $i64;
    }
}
exports.VariantType = VariantType;
class EnumType {
    discriminantType;
    cases;
    case2index;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(cases) {
        this.discriminantType = EnumType.discriminantType(cases.length);
        this.cases = cases;
        this.case2index = new Map();
        for (let i = 0; i < cases.length; i++) {
            const c = cases[i];
            this.case2index.set(c, i);
        }
        this.kind = ComponentModelTypeKind.enum;
        this.size = this.discriminantType.size;
        this.alignment = this.discriminantType.alignment;
        this.flatTypes = this.discriminantType.flatTypes;
    }
    load(memory, offset, context) {
        const index = this.assertRange(this.discriminantType.load(memory, offset, context));
        return this.cases[index];
    }
    liftFlat(memory, values, context) {
        const index = this.assertRange(this.discriminantType.liftFlat(memory, values, context));
        return this.cases[index];
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, value, context) {
        const index = this.case2index.get(value);
        if (index === undefined) {
            throw new ComponentModelTrap('Enumeration value not found');
        }
        this.discriminantType.store(memory, offset, index, context);
    }
    lowerFlat(result, memory, value, context) {
        const index = this.case2index.get(value);
        if (index === undefined) {
            throw new ComponentModelTrap('Enumeration value not found');
        }
        this.discriminantType.lowerFlat(result, memory, index, context);
    }
    copy(dest, dest_offset, src, src_offset, context) {
        this.discriminantType.copy(dest, dest_offset, src, src_offset, context);
    }
    copyFlat(result, dest, values, src, context) {
        this.discriminantType.copyFlat(result, dest, values, src, context);
    }
    assertRange(value) {
        if (value < 0 || value > this.cases.length) {
            throw new ComponentModelTrap('Enumeration value out of range');
        }
        return value;
    }
    static discriminantType(cases) {
        switch (Math.ceil(Math.log2(cases) / 8)) {
            case 0: return u8;
            case 1: return u8;
            case 2: return u16;
            case 3: return u32;
        }
        throw new ComponentModelTrap(`Too many cases: ${cases}`);
    }
}
exports.EnumType = EnumType;
var option;
(function (option) {
    option.none = 'none';
    function None() {
        return new OptionImpl(option.none, undefined);
    }
    option.None = None;
    option.some = 'some';
    function Some(value) {
        return new OptionImpl(option.some, value);
    }
    option.Some = Some;
    function _ctor(c, v) {
        return new OptionImpl(c, v);
    }
    option._ctor = _ctor;
    function isOption(value) {
        return value instanceof OptionImpl;
    }
    option.isOption = isOption;
    class OptionImpl {
        _tag;
        _value;
        constructor(tag, value) {
            this._tag = tag;
            this._value = value;
        }
        get tag() {
            return this._tag;
        }
        get value() {
            return this._value;
        }
        isNone() {
            return this._tag === option.none;
        }
        isSome() {
            return this._tag === option.some;
        }
    }
    class Error extends ResultError {
        constructor(cause) {
            super(`Error value: ${JSON.stringify(cause)}`, cause);
        }
    }
    option.Error = Error;
})(option || (exports.option = option = {}));
class OptionType {
    valueType;
    kind;
    size;
    alignment;
    flatTypes;
    constructor(valueType) {
        this.valueType = valueType;
        this.kind = ComponentModelTypeKind.option;
        this.size = this.computeSize();
        this.alignment = this.computeAlignment();
        this.flatTypes = this.computeFlatTypes();
    }
    load(memory, offset, context) {
        const caseIndex = u8.load(memory, offset);
        if (caseIndex === 0) { // index 0 is none
            return context.options.keepOption ? option._ctor(option.none, undefined) : undefined;
        }
        else {
            offset += u8.size;
            offset = align(offset, this.alignment);
            const value = this.valueType.load(memory, offset, context);
            return (context.options.keepOption ? option._ctor(option.some, value) : value);
        }
    }
    liftFlat(memory, values, context) {
        // First one is the discriminant type. So skip it.
        const caseIndex = u8.liftFlat(memory, values);
        if (caseIndex === 0) { // Index 0 is none
            // Read over the value params
            for (let i = 0; i < this.valueType.flatTypes.length; i++) {
                values.next();
            }
            return context.options.keepOption ? option._ctor(option.none, undefined) : undefined;
        }
        else {
            const value = this.valueType.liftFlat(memory, values, context);
            return (context.options.keepOption ? option._ctor(option.some, value) : value);
        }
    }
    alloc(memory) {
        return memory.alloc(this.alignment, this.size);
    }
    store(memory, offset, value, context) {
        const optValue = this.asOptionValue(value, context.options);
        const index = optValue.tag === option.none ? 0 : 1;
        u8.store(memory, offset, index);
        offset += u8.size;
        if (optValue.tag === option.some) {
            offset = align(offset, this.valueType.alignment);
            this.valueType.store(memory, offset, optValue.value, context);
        }
    }
    lowerFlat(result, memory, value, context) {
        const optValue = this.asOptionValue(value, context.options);
        const index = optValue.tag === option.none ? 0 : 1;
        u8.lowerFlat(result, memory, index);
        if (optValue.tag === option.none) {
            for (const type of this.valueType.flatTypes) {
                if (type === $i64) {
                    result.push(0n);
                }
                else {
                    result.push(0);
                }
            }
        }
        else {
            this.valueType.lowerFlat(result, memory, optValue.value, context);
        }
    }
    copy(dest, dest_offset, src, src_offset, context) {
        u8.copy(dest, dest_offset, src, src_offset);
        const caseIndex = u8.load(src, src_offset);
        if (caseIndex === 0) {
            return;
        }
        else {
            src_offset += u8.size;
            src_offset = align(src_offset, this.alignment);
            dest_offset += u8.size;
            dest_offset = align(dest_offset, this.alignment);
            this.valueType.copy(dest, dest_offset, src, src_offset, context);
        }
    }
    copyFlat(result, dest, values, src, context) {
        u8.copyFlat(result, dest, values, src);
        const caseIndex = result[result.length - 1];
        if (caseIndex === 0) {
            for (const _type of this.valueType.flatTypes) {
                result.push(values.next().value);
            }
        }
        else {
            this.valueType.copyFlat(result, dest, values, src, context);
        }
    }
    asOptionValue(value, options) {
        if (option.isOption(value)) {
            if (!options.keepOption) {
                throw new ComponentModelTrap('Received an option value although options should be unpacked.');
            }
            return value;
        }
        else {
            if (options.keepOption) {
                throw new ComponentModelTrap('Received a unpacked option value although options should NOT be unpacked.');
            }
            return value === undefined ? option._ctor(option.none, undefined) : option._ctor(option.some, value);
        }
    }
    computeSize() {
        let result = u8.size;
        result = align(result, this.valueType.alignment);
        return result + this.valueType.size;
    }
    computeAlignment() {
        return Math.max(u8.alignment, this.valueType.alignment);
    }
    computeFlatTypes() {
        return [...u8.flatTypes, ...this.valueType.flatTypes];
    }
}
exports.OptionType = OptionType;
var result;
(function (result) {
    result.ok = 'ok';
    function Ok(value) {
        return new ResultImpl(result.ok, value);
    }
    result.Ok = Ok;
    result.error = 'error';
    function Error(value) {
        return new ResultImpl(result.error, value);
    }
    result.Error = Error;
    function _ctor(c, v) {
        return new ResultImpl(c, v);
    }
    result._ctor = _ctor;
    class ResultImpl {
        _tag;
        _value;
        constructor(tag, value) {
            this._tag = tag;
            this._value = value;
        }
        get tag() {
            return this._tag;
        }
        get value() {
            return this._value;
        }
        isOk() {
            return this._tag === result.ok;
        }
        isError() {
            return this._tag === result.error;
        }
    }
    result.ResultImpl = ResultImpl;
})(result || (exports.result = result = {}));
class ResultType extends VariantType {
    _errorClass;
    constructor(okType, errorType, errorClass) {
        super([['ok', okType], ['error', errorType]], (result._ctor), ComponentModelTypeKind.result);
        this._errorClass = errorClass;
    }
    get errorClass() {
        return this._errorClass;
    }
}
exports.ResultType = ResultType;
var Resource;
(function (Resource) {
    class Default {
        _handle;
        constructor(handleOrManager) {
            if (typeof handleOrManager === 'number') {
                this._handle = handleOrManager;
            }
            else {
                this._handle = handleOrManager.registerResource(this);
            }
        }
        $handle() {
            return this._handle;
        }
    }
    Resource.Default = Default;
    function getRepresentation(resource) {
        return typeof resource.$rep === 'function' ? resource.$rep() : undefined;
    }
    Resource.getRepresentation = getRepresentation;
})(Resource || (exports.Resource = Resource = {}));
var ResourceManager;
(function (ResourceManager) {
    ResourceManager.handleTag = Symbol('handleTag');
    class Default {
        handleCounter;
        handleTable;
        h2r;
        finalizer;
        ctor;
        dtor;
        // We only need the representation counter for the loop implementation.
        // To make them distinguishable from handles or normal representations we
        // start with MaxValue and decrement it for each new representation.
        representationCounter;
        loopTable;
        constructor() {
            this.handleCounter = 1;
            this.handleTable = new Map();
            this.h2r = new Map();
            this.finalizer = new FinalizationRegistry((value) => {
                const { handle, rep } = value;
                // A proxy was collected, remove the resource.
                try {
                    this.dtor(rep);
                }
                catch (error) {
                    // Log the error.
                    (0, ral_1.default)().console.error(error);
                }
                // Clean up tables
                this.h2r.delete(handle);
                this.handleTable.delete(handle);
                // Also remove the representation from the loop if existed
                this.loopTable?.delete(rep);
            });
            this.representationCounter = Number.MAX_VALUE;
            this.loopTable = undefined;
        }
        newHandle(rep) {
            const handle = this.handleCounter++;
            this.handleTable.set(handle, rep);
            return handle;
        }
        getRepresentation(handle) {
            const rep = this.handleTable.get(handle);
            if (rep === undefined) {
                throw new ComponentModelTrap(`No representation registered for resource handle ${handle}`);
            }
            return rep;
        }
        dropHandle(handle) {
            const rep = this.handleTable.get(handle);
            if (rep === undefined) {
                throw new ComponentModelTrap(`Unknown resource handle ${handle}`);
            }
            if (this.dtor !== undefined) {
                this.dtor(rep);
            }
            this.handleTable.delete(handle);
            return rep;
        }
        setProxyInfo(ctor, dtor) {
            this.ctor = ctor;
            this.dtor = dtor;
        }
        hasResource(handle) {
            return this.h2r.has(handle);
        }
        getResource(handle) {
            const value = this.h2r.get(handle);
            if (value !== undefined) {
                if (value instanceof WeakRef) {
                    const unwrapped = value.deref();
                    if (unwrapped === undefined) {
                        throw new ComponentModelTrap(`Resource for handle ${handle} has been collected.`);
                    }
                    return unwrapped;
                }
                else {
                    return value;
                }
            }
            // This handle represents a resource that lives on the
            // WebAssembly side. Since we don't have a resource for it
            // yet we create one on the fly.
            const rep = this.handleTable.get(handle);
            if (rep !== undefined) {
                if (this.ctor === undefined) {
                    throw new ComponentModelTrap(`No proxy constructor set`);
                }
                const proxy = new this.ctor(ResourceManager.handleTag, handle, rep);
                this.setProxy(handle, rep, proxy);
                return proxy;
            }
            else {
                throw new ComponentModelTrap(`Unknown resource handle ${handle}`);
            }
        }
        registerResource(resource, handle) {
            if (handle !== undefined) {
                if (handle >= this.handleCounter) {
                    throw new ComponentModelTrap(`Handle ${handle} is out of bounds. Current handle counter is ${this.handleCounter}.`);
                }
                if (this.h2r.has(handle)) {
                    throw new ComponentModelTrap(`Handle ${handle} is already registered.`);
                }
                if (this.handleTable.has(handle)) {
                    throw new ComponentModelTrap(`Handle ${handle} is already in use as a proxy handle.`);
                }
            }
            else {
                handle = this.handleCounter++;
            }
            this.h2r.set(handle, resource);
            return handle;
        }
        registerProxy(proxy) {
            const handle = proxy.$handle();
            const rep = Resource.getRepresentation(proxy) ?? this.handleTable.get(handle);
            if (rep === undefined) {
                throw new ComponentModelTrap(`Unknown proxy handle ${handle}`);
            }
            this.setProxy(handle, rep, proxy);
        }
        removeResource(value) {
            const handle = typeof value === 'number' ? value : value.$handle();
            const resource = this.h2r.get(handle);
            if (resource === undefined) {
                throw new ComponentModelTrap(`Unknown resource handle ${handle}.`);
            }
            if (resource instanceof WeakRef) {
                throw new ComponentModelTrap(`Proxy resources should not be removed manually. They are removed via the GC.`);
            }
            this.h2r.delete(handle);
        }
        registerLoop(handle) {
            if (this.loopTable === undefined) {
                this.loopTable = new Map();
            }
            const result = this.handleCounter++;
            const representation = this.representationCounter--;
            this.handleTable.set(result, representation);
            this.loopTable.set(representation, handle);
            return result;
        }
        getLoop(rep) {
            const result = this.loopTable?.get(rep);
            if (result === undefined) {
                throw new ComponentModelTrap(`Unknown loop handle for representation ${rep}`);
            }
            return result;
        }
        setProxy(handle, rep, proxy) {
            if (this.dtor === undefined) {
                throw new ComponentModelTrap(`No proxy destructor set`);
            }
            this.h2r.set(handle, new WeakRef(proxy));
            this.finalizer.register(proxy, { handle, rep }, proxy);
        }
    }
    ResourceManager.Default = Default;
    function from(obj) {
        if (obj === undefined) {
            return undefined;
        }
        return (obj.$resources ?? obj.$resourceManager ?? obj.$manager);
    }
    ResourceManager.from = from;
})(ResourceManager || (exports.ResourceManager = ResourceManager = {}));
var ResourceManagers;
(function (ResourceManagers) {
    class Default {
        managers;
        constructor() {
            this.managers = new Map();
        }
        has(id) {
            return this.managers.has(id);
        }
        set(id, manager) {
            if (this.managers.has(id)) {
                throw new ComponentModelTrap(`Resource manager ${id} already registered.`);
            }
            this.managers.set(id, manager);
        }
        ensure(id) {
            const manager = this.managers.get(id);
            if (manager === undefined) {
                throw new ComponentModelTrap(`Resource manager ${id} not found.`);
            }
            return manager;
        }
        get(id) {
            return this.managers.get(id);
        }
    }
    ResourceManagers.Default = Default;
    function is(value) {
        const candidate = value;
        return candidate && typeof candidate.has === 'function' && typeof candidate.ensure === 'function' && typeof candidate.get === 'function' && typeof candidate.set === 'function';
    }
    ResourceManagers.is = is;
})(ResourceManagers || (exports.ResourceManagers = ResourceManagers = {}));
class Callable {
    static EMPTY_JTYPE = Object.freeze([]);
    static EMPTY_WASM_TYPE = Object.freeze([]);
    static MAX_FLAT_PARAMS = 16;
    static MAX_FLAT_RESULTS = 1;
    witName;
    params;
    returnType;
    paramType;
    isSingleParam;
    mode;
    constructor(witName, params, returnType) {
        this.witName = witName;
        this.params = params;
        this.returnType = returnType;
        switch (params.length) {
            case 0:
                this.paramType = undefined;
                this.isSingleParam = false;
                break;
            case 1:
                this.paramType = params[0][1];
                this.isSingleParam = true;
                break;
            default:
                this.paramType = new TupleType(params.map(p => p[1]));
                this.isSingleParam = false;
        }
        this.mode = 'lower';
    }
    liftParamValues(wasmValues, memory, context) {
        if (this.paramType === undefined) {
            return Callable.EMPTY_JTYPE;
        }
        let result;
        if (this.paramType.flatTypes.length > Callable.MAX_FLAT_PARAMS) {
            const p0 = wasmValues[0];
            if (!Number.isInteger(p0)) {
                throw new ComponentModelTrap('Invalid pointer');
            }
            result = this.paramType.load(memory.readonly(p0, this.paramType.size), 0, context);
        }
        else {
            result = this.paramType.liftFlat(memory, wasmValues.values(), context);
        }
        return this.isSingleParam ? [result] : result;
    }
    lowerParamValues(values, memory, context) {
        if (this.paramType === undefined) {
            return Callable.EMPTY_WASM_TYPE;
        }
        if (this.isSingleParam && values.length !== 1) {
            throw new ComponentModelTrap(`Expected a single parameter, but got ${values.length}`);
        }
        const toLower = this.isSingleParam ? values[0] : values;
        if (this.paramType.flatTypes.length > Callable.MAX_FLAT_PARAMS) {
            const writer = this.paramType.alloc(memory);
            this.paramType.store(writer, 0, toLower, context);
            return [writer.ptr];
        }
        else {
            const result = [];
            this.paramType.lowerFlat(result, memory, toLower, context);
            return result;
        }
    }
    copyParamValues(result, dest, wasmValues, src, context) {
        const flatReturnTypes = this.returnType !== undefined ? this.returnType.flatTypes.length : 0;
        const flatParamTypes = this.paramType !== undefined ? this.paramType.flatTypes.length : 0;
        let out = undefined;
        if (flatReturnTypes > Callable.MAX_FLAT_RESULTS) {
            // Check if the result pointer got passed as the last value in the flat types
            if (wasmValues.length === flatParamTypes + 1) {
                const last = wasmValues[flatParamTypes];
                if (!Number.isInteger(last)) {
                    throw new ComponentModelTrap(`Expected a pointer as return parameter, but got ${last}`);
                }
                out = last;
            }
        }
        if (this.paramType === undefined) {
            if ((out === undefined && wasmValues.length !== 0) || (out !== undefined && wasmValues.length !== 1)) {
                throw new ComponentModelTrap(`Expected no parameters, but got ${wasmValues.length}`);
            }
        }
        else if (this.paramType.flatTypes.length > Callable.MAX_FLAT_PARAMS) {
            const p0 = wasmValues[0];
            if (!Number.isInteger(p0)) {
                throw new ComponentModelTrap('Invalid pointer');
            }
            const srcReader = src.readonly(p0, this.paramType.size);
            this.paramType.copy(this.paramType.alloc(dest), 0, srcReader, 0, context);
        }
        else {
            this.paramType.copyFlat(result, dest, wasmValues.values(), src, context);
        }
        // Allocate space for the result in dest and add it to the end of the flat values.
        if (out !== undefined && this.returnType !== undefined) {
            const destResult = this.returnType.alloc(dest);
            result.push(destResult.ptr);
            return { transferResult: destResult, originalResult: src.preAllocated(out, this.returnType.size) };
        }
        else {
            return undefined;
        }
    }
    lowerReturnValue(value, memory, context, out) {
        if (this.returnType === undefined) {
            return;
        }
        if (this.returnType.flatTypes.length <= Callable.MAX_FLAT_RESULTS) {
            const result = [];
            this.returnType.lowerFlat(result, memory, value, context);
            if (result.length !== this.returnType.flatTypes.length) {
                throw new ComponentModelTrap(`Expected flat result of length ${this.returnType.flatTypes.length}, but got ${JSON.stringify(result, undefined, undefined)}`);
            }
            return result[0];
        }
        else {
            const writer = out !== undefined ? memory.preAllocated(out, this.returnType.size) : this.returnType.alloc(memory);
            this.returnType.store(writer, 0, value, context);
            return out !== undefined ? undefined : writer.ptr;
        }
    }
    handleError(error, memory, context, out) {
        if (!(this.returnType instanceof ResultType) || this.returnType.errorClass === undefined || !(error instanceof this.returnType.errorClass)) {
            throw error;
        }
        const value = result.Error(error.cause);
        return this.lowerReturnValue(value, memory, context, out);
    }
    copyReturnValue(resultStorage, dest, src, value, context) {
        if (resultStorage !== undefined) {
            if (this.returnType === undefined) {
                throw new ComponentModelTrap(`Result storage should not be set if there is no return type.`);
            }
            if (value !== undefined) {
                throw new ComponentModelTrap(`Can't use both result storage and result value ${value}.`);
            }
            this.returnType.copy(resultStorage.originalResult, 0, resultStorage.transferResult, 0, context);
            return undefined;
        }
        else if (value !== undefined) {
            if (this.returnType === undefined) {
                throw new ComponentModelTrap(`Expected no return value, but got ${value}`);
            }
            if (this.returnType.flatTypes.length > Callable.MAX_FLAT_RESULTS) {
                if (!Number.isInteger(value)) {
                    throw new ComponentModelTrap(`Expected a pointer as return value, but got ${value}`);
                }
                const destWriter = this.returnType.alloc(dest);
                this.returnType.copy(destWriter, 0, src.preAllocated(value, this.returnType.size), 0, context);
                return destWriter.ptr;
            }
            else {
                return value;
            }
        }
        else {
            return undefined;
        }
    }
    /**
     * Calls a function inside a wasm module.
     */
    callWasm(params, wasmFunction, context) {
        const memory = context.getMemory();
        const wasmValues = this.lowerParamValues(params, memory, context);
        const result = wasmFunction(...wasmValues);
        return this.liftReturnValue(result, memory, context);
    }
    /**
     * Calls a resource method inside a wasm module.
     */
    callWasmMethod(obj, params, wasmFunction, context) {
        const memory = context.getMemory();
        const handle = obj.$rep();
        const wasmValues = this.lowerParamValues(params, memory, context);
        const result = wasmFunction(handle, ...wasmValues);
        return this.liftReturnValue(result, memory, context);
    }
    /**
     * Call a host function on the main thread from a wasm module.
     */
    callMain(connection, qualifier, params, context) {
        connection.prepareCall();
        const newParams = [];
        const resultStorage = this.copyParamValues(newParams, connection.getMemory(), params, context.getMemory(), context);
        const result = connection.callMain(`${qualifier}#${this.witName}`, newParams);
        return this.copyReturnValue(resultStorage, context.getMemory(), connection.getMemory(), result, context);
    }
    /**
     * Call a wasm function from a worker thread.
     */
    callWasmFromWorker(transferMemory, func, params, context) {
        const newParams = [];
        const resultStorage = this.copyParamValues(newParams, context.getMemory(), params, transferMemory, context);
        const result = func(...newParams);
        return this.copyReturnValue(resultStorage, transferMemory, context.getMemory(), result, context);
    }
    /**
     * Call a wasm method from a worker thread.
     */
    callWasmMethodFromWorker(transferMemory, func, params, context) {
        const handle = params[0];
        if (typeof handle !== 'number') {
            throw new ComponentModelTrap(`Expected a number as handle, but got ${handle}`);
        }
        const newParams = [];
        const resultStorage = this.copyParamValues(newParams, context.getMemory(), params.slice(1), transferMemory, context);
        const result = func(handle, ...newParams);
        return this.copyReturnValue(resultStorage, transferMemory, context.getMemory(), result, context);
    }
    /**
     * Call the wasm function from the main thread.
     */
    async callWorker(connection, qualifier, params, context) {
        return connection.lock(async () => {
            connection.prepareCall();
            const memory = connection.getMemory();
            const wasmValues = this.lowerParamValues(params, memory, context);
            let result = await connection.callWorker(`${qualifier}#${this.witName}`, wasmValues);
            return this.liftReturnValue(result, memory, context);
        });
    }
    /**
     * Call a resource method inside a wasm module.
     */
    async callWorkerMethod(connection, qualifier, obj, params, context) {
        return connection.lock(async () => {
            connection.prepareCall();
            const memory = connection.getMemory();
            const handle = obj.$rep();
            const wasmValues = this.lowerParamValues(params, memory, context).slice();
            wasmValues.unshift(handle);
            const result = await connection.callWorker(`${qualifier}#${this.witName}`, wasmValues);
            return this.liftReturnValue(result, memory, context);
        });
    }
    getParamValuesForHostCall(params, memory, context) {
        const returnFlatTypes = this.returnType === undefined ? 0 : this.returnType.flatTypes.length;
        // We currently only support 'lower' mode for results > MAX_FLAT_RESULTS. From the spec:
        // As an optimization, when lowering the return value of an imported function (via canon lower),
        // the caller can have already allocated space for the return value (e.g., efficiently on the stack),
        // passing in an i32 pointer as an parameter instead of returning an i32 as a return value.
        // See https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md#flattening
        let out;
        if (returnFlatTypes > FunctionType.MAX_FLAT_RESULTS) {
            const paramFlatTypes = this.paramType !== undefined ? this.paramType.flatTypes.length : 0;
            // The caller allocated the memory. We just need to pass the pointer.
            if (params.length === paramFlatTypes + 1) {
                const last = params[paramFlatTypes];
                if (typeof last !== 'number') {
                    throw new ComponentModelTrap(`Result pointer must be a number (u32), but got ${out}.`);
                }
                out = last;
            }
        }
        return [this.liftParamValues(params, memory, context), out];
    }
    liftReturnValue(value, memory, context) {
        if (this.returnType === undefined) {
            return;
        }
        let result;
        if (this.returnType.flatTypes.length <= Callable.MAX_FLAT_RESULTS) {
            result = this.returnType.liftFlat(memory, [value].values(), context);
        }
        else {
            result = this.returnType.load(memory.readonly(value, this.returnType.size), 0, context);
        }
        if (this.returnType instanceof ResultType) {
            const resultValue = result;
            if (resultValue.isError()) {
                if (this.returnType.errorClass === undefined) {
                    throw new ComponentModelTrap(`Received an error result, but no error class is defined.`);
                }
                throw new this.returnType.errorClass(resultValue.value);
            }
            else {
                return resultValue.value;
            }
        }
        else {
            return result;
        }
    }
}
class FunctionType extends Callable {
    constructor(witName, params, returnType) {
        super(witName, params, returnType);
    }
    /**
     * Calls a service function from a wasm module.
     */
    callService(func, params, context) {
        const [jParams, out] = this.getParamValuesForHostCall(params, context.getMemory(), context);
        try {
            const result = func(...jParams);
            return this.lowerReturnValue(result, context.getMemory(), context, out);
        }
        catch (error) {
            return this.handleError(error, context.getMemory(), context, out);
        }
    }
    async callServiceAsync(memory, func, params, context) {
        const [jParams, out] = this.getParamValuesForHostCall(params, memory, context);
        try {
            const result = await func(...jParams);
            return this.lowerReturnValue(result, memory, context, out);
        }
        catch (error) {
            return this.handleError(error, memory, context, out);
        }
    }
}
exports.FunctionType = FunctionType;
class ConstructorType extends Callable {
    constructor(witName, params, returnType) {
        super(witName, params, returnType);
    }
    callService(clazz, params, context) {
        // We currently only support 'lower' mode for results > MAX_FLAT_RESULTS.
        const returnFlatTypes = this.returnType === undefined ? 0 : this.returnType.flatTypes.length;
        if (returnFlatTypes !== 1) {
            throw new ComponentModelTrap(`Expected exactly one return type, but got ${returnFlatTypes}.`);
        }
        const memory = context.getMemory();
        const jParams = this.liftParamValues(params, memory, context);
        const obj = new clazz(...jParams);
        return obj.$handle();
    }
    async callServiceAsync(memory, clazz, params, context) {
        // We currently only support 'lower' mode for results > MAX_FLAT_RESULTS.
        const returnFlatTypes = this.returnType === undefined ? 0 : this.returnType.flatTypes.length;
        if (returnFlatTypes !== 1) {
            throw new ComponentModelTrap(`Expected exactly one return type, but got ${returnFlatTypes}.`);
        }
        const jParams = this.liftParamValues(params, memory, context);
        const obj = await clazz.$new(...jParams);
        return obj.$handle();
    }
    callWasmConstructor(params, wasmFunction, context) {
        const memory = context.getMemory();
        const wasmValues = this.lowerParamValues(params, memory, context);
        const result = wasmFunction(...wasmValues);
        if (typeof result !== 'number') {
            throw new ComponentModelTrap(`Expected a number (u32) as return value, but got ${result}.`);
        }
        return result;
    }
    callWasmConstructorAsync(connection, qualifier, params, context) {
        return connection.lock(async () => {
            connection.prepareCall();
            const memory = connection.getMemory();
            const wasmValues = this.lowerParamValues(params, memory, context);
            return connection.callWorker(`${qualifier}#${this.witName}`, wasmValues);
        });
    }
}
exports.ConstructorType = ConstructorType;
class DestructorType extends Callable {
    constructor(witName, params) {
        super(witName, params);
    }
    callService(params, resourceManager) {
        const handle = params[0];
        if (typeof handle === 'bigint' || !u32.valid(handle)) {
            throw new ComponentModelTrap(`Object handle must be a number (u32), but got ${handle}.`);
        }
        const resource = resourceManager.getResource(handle);
        resource['$drop'] !== undefined && resource['$drop']();
        resourceManager.removeResource(handle);
    }
    async callServiceAsync(_memory, params, resourceManager) {
        const handle = params[0];
        if (typeof handle === 'bigint' || !u32.valid(handle)) {
            throw new ComponentModelTrap(`Object handle must be a number (u32), but got ${handle}.`);
        }
        const resource = resourceManager.getResource(handle);
        resource['$drop'] !== undefined && await resource['$drop']();
        resourceManager.removeResource(handle);
    }
}
exports.DestructorType = DestructorType;
class StaticMethodType extends Callable {
    constructor(witName, params, returnType) {
        super(witName, params, returnType);
    }
    callService(func, params, context) {
        const [jParams, out] = this.getParamValuesForHostCall(params, context.getMemory(), context);
        try {
            const result = func(...jParams);
            return this.lowerReturnValue(result, context.getMemory(), context, out);
        }
        catch (error) {
            return this.handleError(error, context.getMemory(), context, out);
        }
    }
    async callServiceAsync(memory, func, params, context) {
        const [jParams, out] = this.getParamValuesForHostCall(params, memory, context);
        try {
            const result = await func(...jParams);
            return this.lowerReturnValue(result, memory, context, out);
        }
        catch (error) {
            return this.handleError(error, memory, context, out);
        }
    }
}
exports.StaticMethodType = StaticMethodType;
class MethodType extends Callable {
    constructor(witName, params, returnType) {
        super(witName, params, returnType);
    }
    callService(methodName, params, resourceManager, context) {
        if (params.length === 0) {
            throw new ComponentModelTrap(`Method calls must have at least one parameter (the object pointer).`);
        }
        // We need to cut off the first parameter (the object handle).
        const handle = params.shift();
        if (typeof handle !== 'number') {
            throw new ComponentModelTrap(`Object handle must be a number (u32), but got ${handle}.`);
        }
        const [jParams, out] = this.getParamValuesForHostCall(params, context.getMemory(), context);
        const resource = resourceManager.getResource(handle);
        const memory = context.getMemory();
        try {
            const result = resource[methodName](...jParams);
            return this.lowerReturnValue(result, memory, context, out);
        }
        catch (error) {
            return this.handleError(error, memory, context, out);
        }
    }
    async callServiceAsync(memory, methodName, params, resourceManager, context) {
        if (params.length === 0) {
            throw new ComponentModelTrap(`Method calls must have at least one parameter (the object pointer).`);
        }
        // We need to cut off the first parameter (the object handle).
        const handle = params.shift();
        if (typeof handle !== 'number') {
            throw new ComponentModelTrap(`Object handle must be a number (u32), but got ${handle}.`);
        }
        const [jParams, out] = this.getParamValuesForHostCall(params, memory, context);
        const resource = resourceManager.getResource(handle);
        try {
            const result = await resource[methodName](...jParams);
            return this.lowerReturnValue(result, memory, context, out);
        }
        catch (error) {
            return this.handleError(error, memory, context, out);
        }
    }
}
exports.MethodType = MethodType;
class ResourceHandleType {
    kind;
    size;
    alignment;
    flatTypes;
    witName;
    constructor(witName) {
        this.witName = witName;
        this.kind = ComponentModelTypeKind.resourceHandle;
        this.size = u32.size;
        this.alignment = u32.alignment;
        this.flatTypes = u32.flatTypes;
    }
    load(memory, offset) {
        return u32.load(memory, offset);
    }
    liftFlat(memory, values) {
        return u32.liftFlat(memory, values);
    }
    alloc(memory) {
        return u32.alloc(memory);
    }
    store(memory, offset, value) {
        u32.store(memory, offset, value);
    }
    lowerFlat(result, memory, value) {
        u32.lowerFlat(result, memory, value);
    }
    copy(dest, dest_offset, src, src_offset) {
        u32.copy(dest, dest_offset, src, src_offset);
    }
    copyFlat(result, dest, values, src) {
        u32.copyFlat(result, dest, values, src);
    }
}
exports.ResourceHandleType = ResourceHandleType;
class ResourceType {
    kind;
    size;
    alignment;
    flatTypes;
    witName;
    id;
    callables;
    constructor(witName, id) {
        this.kind = ComponentModelTypeKind.resource;
        this.size = u32.size;
        this.alignment = u32.alignment;
        this.flatTypes = u32.flatTypes;
        this.witName = witName;
        this.id = id;
        this.callables = new Map();
    }
    addConstructor(jsName, func) {
        this.callables.set(jsName, func);
    }
    addDestructor(jsName, func) {
        this.callables.set(jsName, func);
    }
    addStaticMethod(jsName, func) {
        this.callables.set(jsName, func);
    }
    addMethod(jsName, func) {
        this.callables.set(jsName, func);
    }
    getCallable(jsName) {
        const result = this.callables.get(jsName);
        if (result === undefined) {
            throw new ComponentModelTrap(`Method '${jsName}' not found on resource '${this.witName}'.`);
        }
        return result;
    }
    load(memory, offset, context) {
        const handle = u32.load(memory, offset);
        return context.resources.ensure(this.id).getResource(handle);
    }
    liftFlat(memory, values, context) {
        const handle = u32.liftFlat(memory, values);
        return context.resources.ensure(this.id).getResource(handle);
    }
    alloc(memory) {
        return u32.alloc(memory);
    }
    store(memory, offset, value) {
        const handle = value.$handle();
        u32.store(memory, offset, handle);
    }
    lowerFlat(result, memory, value) {
        const handle = value.$handle();
        u32.lowerFlat(result, memory, handle);
    }
    copy(dest, dest_offset, src, src_offset) {
        u32.copy(dest, dest_offset, src, src_offset);
    }
    copyFlat(result, dest, values, src) {
        u32.copyFlat(result, dest, values, src);
    }
}
exports.ResourceType = ResourceType;
class AbstractWrapperType {
    kind;
    size;
    alignment;
    flatTypes;
    wrapped;
    constructor(kind, wrapped) {
        this.kind = kind;
        this.wrapped = wrapped;
        this.size = u32.size;
        this.alignment = u32.alignment;
        this.flatTypes = u32.flatTypes;
    }
    load(memory, offset, context) {
        return this.wrapped.load(memory, offset, context);
    }
    liftFlat(memory, values, context) {
        return this.wrapped.liftFlat(memory, values, context);
    }
    alloc(memory) {
        return u32.alloc(memory);
    }
    store(memory, offset, value, context) {
        return this.wrapped.store(memory, offset, value, context);
    }
    lowerFlat(result, memory, value, context) {
        return this.wrapped.lowerFlat(result, memory, value, context);
    }
    copy(dest, dest_offset, src, src_offset, context) {
        return this.wrapped.copy(dest, dest_offset, src, src_offset, context);
    }
    copyFlat(result, dest, values, src, context) {
        return this.wrapped.copyFlat(result, dest, values, src, context);
    }
}
class BorrowType extends AbstractWrapperType {
    constructor(type) {
        super(ComponentModelTypeKind.borrow, type);
    }
}
exports.BorrowType = BorrowType;
class OwnType extends AbstractWrapperType {
    constructor(type) {
        super(ComponentModelTypeKind.own, type);
    }
}
exports.OwnType = OwnType;
var InterfaceType;
(function (InterfaceType) {
    function is(value) {
        return typeof value === 'object' && typeof value.id === 'string' && typeof value.witName === 'string'
            && value.types instanceof Map && value.functions instanceof Map && value.resources instanceof Map;
    }
    InterfaceType.is = is;
})(InterfaceType || (exports.InterfaceType = InterfaceType = {}));
var PackageType;
(function (PackageType) {
    function is(value) {
        return typeof value === 'object' && typeof value.id === 'string' && typeof value.witName === 'string'
            && value.interfaces instanceof Map;
    }
    PackageType.is = is;
})(PackageType || (exports.PackageType = PackageType = {}));
var WasmContext;
(function (WasmContext) {
    class Default {
        memory;
        options;
        resources;
        constructor(options, resources) {
            this.options = options ?? { encoding: 'utf-8' };
            this.resources = resources ?? new ResourceManagers.Default();
        }
        initialize(memory) {
            if (this.memory !== undefined) {
                throw new MemoryError(`Memory is already initialized.`);
            }
            this.memory = memory;
        }
        getMemory() {
            if (this.memory === undefined) {
                throw new MemoryError(`Memory not yet initialized.`);
            }
            return this.memory;
        }
    }
    WasmContext.Default = Default;
    function is(value) {
        const candidate = value;
        return candidate && typeof candidate.getMemory === 'function' && Options.is(candidate.options) && ResourceManagers.is(candidate.resources);
    }
    WasmContext.is = is;
})(WasmContext || (exports.WasmContext = WasmContext = {}));
function getResourceManager(resource, clazz, context) {
    let resourceManager;
    if (context.resources.has(resource.id)) {
        resourceManager = context.resources.ensure(resource.id);
    }
    else {
        resourceManager = ResourceManager.from(clazz) ?? new ResourceManager.Default();
        context.resources.set(resource.id, resourceManager);
    }
    return resourceManager;
}
var $imports;
(function ($imports) {
    function create(world, service, context) {
        const packageName = world.id.substring(0, world.id.indexOf('/'));
        const result = Object.create(null);
        if (world.imports !== undefined) {
            if (world.imports.functions !== undefined) {
                result['$root'] = doCreate(world.imports.functions, undefined, service, context);
            }
            if (world.imports.interfaces !== undefined) {
                for (const [name, iface] of world.imports.interfaces) {
                    const propName = `${name[0].toLowerCase()}${name.substring(1)}`;
                    result[`${packageName}/${iface.witName}`] = doCreate(iface.functions, iface.resources, service[propName], context);
                }
            }
        }
        if (world.exports !== undefined) {
            if (world.exports.interfaces !== undefined) {
                for (const iface of world.exports.interfaces.values()) {
                    if (iface.resources === undefined) {
                        continue;
                    }
                    for (const resource of iface.resources.values()) {
                        const manager = getResourceManager(resource, undefined, context);
                        const exports = Object.create(null);
                        exports[`[resource-new]${resource.witName}`] = (rep) => manager.newHandle(rep);
                        exports[`[resource-rep]${resource.witName}`] = (handle) => manager.getRepresentation(handle);
                        exports[`[resource-drop]${resource.witName}`] = (handle) => manager.dropHandle(handle);
                        result[`[export]${packageName}/${iface.witName}`] = exports;
                    }
                }
            }
        }
        return result;
    }
    $imports.create = create;
    function loop(world, service, context) {
        const imports = create(world, service, context);
        const wasmExports = asExports(imports, context);
        const loop = {
            id: world.id,
            witName: world.witName,
            imports: world.exports !== undefined ? {
                functions: world.exports.functions,
                interfaces: world.exports.interfaces,
            } : undefined,
            exports: world.imports !== undefined ? {
                functions: world.imports.functions,
                interfaces: world.imports.interfaces,
            } : undefined,
        };
        return $exports.bind(loop, wasmExports, context);
    }
    $imports.loop = loop;
    function asExports(imports, context) {
        const result = Object.create(null);
        const keys = Object.keys(imports);
        for (const ifaceName of keys) {
            const iface = imports[ifaceName];
            if (ifaceName.startsWith('[export]')) {
                continue;
            }
            else if (ifaceName === '$root') {
                for (const funcName of Object.keys(iface)) {
                    result[funcName] = iface[funcName];
                }
            }
            else {
                const qualifier = `${ifaceName}#`;
                for (const funcName of Object.keys(iface)) {
                    if (funcName.startsWith('[constructor]')) {
                        const managerId = `${ifaceName}/${funcName.substring(13 /* length of [constructor] */)}`;
                        const resourceManager = context.resources.ensure(managerId);
                        result[`${qualifier}${funcName}`] = (...args) => {
                            const handle = iface[funcName](...args);
                            return resourceManager.registerLoop(handle);
                        };
                    }
                    else if (funcName.startsWith('[method]')) {
                        let resourceName = funcName.substring(8 /* length of [method] */);
                        if (resourceName.indexOf('.') !== -1) {
                            resourceName = resourceName.substring(0, resourceName.indexOf('.'));
                        }
                        const managerId = `${ifaceName}/${resourceName}`;
                        const resourceManager = context.resources.ensure(managerId);
                        result[`${qualifier}${funcName}`] = ((rep, ...args) => {
                            return iface[funcName](resourceManager.getLoop(rep), ...args);
                        });
                    }
                    else if (funcName.startsWith('[resource-drop]')) {
                        result[`${qualifier}[dtor]${funcName.substring(15 /* length of [resource-drop] */)}`] = iface[funcName];
                    }
                    else {
                        result[`${qualifier}${funcName}`] = iface[funcName];
                    }
                }
            }
        }
        return result;
    }
    function doCreate(functions, resources, service, context) {
        const result = Object.create(null);
        if (functions !== undefined) {
            for (const [funcName, func] of functions) {
                result[func.witName] = createFunction(func, service[funcName], context);
            }
        }
        if (resources !== undefined) {
            for (const [resourceName, resource] of resources) {
                const clazz = service[resourceName];
                const resourceManager = getResourceManager(resource, clazz, context);
                for (const [callableName, callable] of resource.callables) {
                    if (callable instanceof ConstructorType) {
                        result[callable.witName] = createConstructorFunction(callable, clazz, context);
                    }
                    else if (callable instanceof StaticMethodType) {
                        result[callable.witName] = createStaticMethodFunction(callable, service[resourceName][callableName], context);
                    }
                    else if (callable instanceof MethodType) {
                        result[callable.witName] = createMethodFunction(callableName, callable, resourceManager, context);
                    }
                    else if (callable instanceof DestructorType) {
                        result[callable.witName] = createDestructorFunction(callable, resourceManager);
                    }
                }
            }
        }
        return result;
    }
    function createFunction(callable, serviceFunction, context) {
        return function (...params) {
            return callable.callService(serviceFunction, params, context);
        };
    }
    function createConstructorFunction(callable, clazz, context) {
        return function (...params) {
            return callable.callService(clazz, params, context);
        };
    }
    function createDestructorFunction(callable, manager) {
        return function (...params) {
            return callable.callService(params, manager);
        };
    }
    function createStaticMethodFunction(callable, func, context) {
        return function (...params) {
            return callable.callService(func, params, context);
        };
    }
    function createMethodFunction(name, callable, manager, context) {
        return function (...params) {
            return callable.callService(name, params, manager, context);
        };
    }
    let worker;
    (function (worker) {
        function create(connection, world, context) {
            const packageName = world.id.substring(0, world.id.indexOf('/'));
            const result = Object.create(null);
            if (world.imports !== undefined) {
                if (world.imports.functions !== undefined) {
                    result['$root'] = doCreate(connection, '$root', world.imports.functions, undefined, context);
                }
                if (world.imports.interfaces !== undefined) {
                    for (const iface of world.imports.interfaces.values()) {
                        const qualifier = `${packageName}/${iface.witName}`;
                        result[qualifier] = doCreate(connection, qualifier, iface.functions, iface.resources, context);
                    }
                }
            }
            if (world.exports !== undefined) {
                if (world.exports.interfaces !== undefined) {
                    for (const iface of world.exports.interfaces.values()) {
                        if (iface.resources === undefined) {
                            continue;
                        }
                        for (const resource of iface.resources.values()) {
                            const exports = Object.create(null);
                            const qualifier = `[export]${packageName}/${iface.witName}`;
                            const newName = `[resource-new]${resource.witName}`;
                            exports[newName] = (rep) => connection.callMain(`${qualifier}#${newName}`, [rep]);
                            const repName = `[resource-rep]${resource.witName}`;
                            exports[repName] = (handle) => connection.callMain(`${qualifier}#${repName}`, [handle]);
                            const dropName = `[resource-drop]${resource.witName}`;
                            exports[dropName] = (handle) => connection.callMain(`${qualifier}#${dropName}`, [handle]);
                            result[qualifier] = exports;
                        }
                    }
                }
            }
            return result;
        }
        worker.create = create;
        function doCreate(connection, qualifier, functions, resources, context) {
            const result = Object.create(null);
            if (functions !== undefined) {
                for (const [, func] of functions) {
                    result[func.witName] = function (...params) {
                        return func.callMain(connection, qualifier, params, context);
                    };
                }
            }
            if (resources !== undefined) {
                for (const resource of resources.values()) {
                    for (const callable of resource.callables.values()) {
                        result[callable.witName] = function (...params) {
                            return callable.callMain(connection, qualifier, params, context);
                        };
                    }
                }
            }
            return result;
        }
    })(worker = $imports.worker || ($imports.worker = {}));
})($imports || (exports.$imports = $imports = {}));
var $exports;
(function ($exports) {
    function bind(world, exports, context) {
        const [root, scoped] = partition(exports);
        const result = Object.create(null);
        if (world.exports !== undefined) {
            if (world.exports.functions !== undefined) {
                Object.assign(result, doBind(world.exports.functions, undefined, root, context));
            }
            if (world.exports.interfaces !== undefined) {
                for (const [name, iface] of world.exports.interfaces) {
                    const propName = `${name[0].toLowerCase()}${name.substring(1)}`;
                    result[propName] = doBind(iface.functions, iface.resources, scoped[iface.id], context);
                }
            }
        }
        return result;
    }
    $exports.bind = bind;
    function partition(exports) {
        const root = Object.create(null);
        const scoped = Object.create(null);
        for (const [key, value] of Object.entries(exports)) {
            const parts = key.split('#');
            if (parts.length === 1) {
                root[key] = value;
            }
            else {
                const [iface, func] = parts;
                if (scoped[iface] === undefined) {
                    scoped[iface] = Object.create(null);
                }
                scoped[iface][func] = value;
            }
        }
        return [root, scoped];
    }
    function doBind(functions, resources, wasm, context) {
        const result = Object.create(null);
        if (functions !== undefined) {
            for (const [name, func] of functions) {
                result[name] = createFunction(func, wasm[func.witName], context);
            }
        }
        if (resources !== undefined) {
            for (const [name, resource] of resources) {
                const resourceManager = getResourceManager(resource, undefined, context);
                const cl = clazz.create(resource, wasm, context);
                resourceManager.setProxyInfo(cl, wasm[`[dtor]${resource.witName}`]);
                result[name] = cl;
            }
        }
        return result;
    }
    function createFunction(func, wasmFunction, context) {
        return (...params) => {
            return func.callWasm(params, wasmFunction, context);
        };
    }
    let worker;
    (function (worker) {
        function bind(connection, world, exports, context) {
            const packageName = world.id.substring(0, world.id.indexOf('/'));
            const [root, scoped] = partition(exports);
            if (world.exports !== undefined) {
                doBind(connection, packageName, world.exports.functions, undefined, root, context);
                if (world.exports.interfaces !== undefined) {
                    for (const iface of world.exports.interfaces.values()) {
                        doBind(connection, `${packageName}/${iface.witName}`, iface.functions, iface.resources, scoped[iface.id], context);
                    }
                }
            }
        }
        worker.bind = bind;
        function doBind(connection, qualifier, functions, resources, exports, context) {
            if (functions !== undefined) {
                for (const func of functions.values()) {
                    connection.on(`${qualifier}#${func.witName}`, (memory, params) => {
                        return func.callWasmFromWorker(memory, exports[func.witName], params, context);
                    });
                }
            }
            if (resources !== undefined) {
                for (const resource of resources.values()) {
                    for (const callable of resource.callables.values()) {
                        if (callable instanceof ConstructorType || callable instanceof StaticMethodType) {
                            connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                                return callable.callWasmFromWorker(memory, exports[callable.witName], params, context);
                            });
                        }
                        else {
                            connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                                return callable.callWasmMethodFromWorker(memory, exports[callable.witName], params, context);
                            });
                        }
                    }
                }
            }
        }
    })(worker = $exports.worker || ($exports.worker = {}));
})($exports || (exports.$exports = $exports = {}));
var clazz;
(function (clazz_1) {
    function create(resource, wasm, context) {
        let resourceManager;
        if (context.resources.has(resource.id)) {
            resourceManager = context.resources.ensure(resource.id);
        }
        else {
            resourceManager = new ResourceManager.Default();
            context.resources.set(resource.id, resourceManager);
        }
        const clazz = class extends Resource.Default {
            _rep;
            constructor(...args) {
                if (args[0] === ResourceManager.handleTag) {
                    const handle = args[1];
                    super(handle);
                    this._rep = args[2];
                }
                else {
                    const ctor = resource.getCallable('constructor');
                    const handle = ctor.callWasmConstructor(args, wasm[ctor.witName], context);
                    super(handle);
                    this._rep = resourceManager.getRepresentation(this.$handle());
                }
            }
            $rep() {
                return this._rep;
            }
        };
        for (const [name, callable] of resource.callables) {
            if (callable instanceof MethodType) {
                clazz.prototype[name] = function (...params) {
                    return callable.callWasmMethod(this, params, wasm[callable.witName], context);
                };
            }
            else if (callable instanceof DestructorType) {
                clazz.prototype[name] = function (...params) {
                    return callable.callWasmMethod(this, params, wasm[callable.witName], context);
                };
            }
            else if (callable instanceof StaticMethodType) {
                clazz[name] = (...params) => {
                    return callable.callWasm(params, wasm[callable.witName], context);
                };
            }
        }
        return clazz;
    }
    clazz_1.create = create;
    function createPromise(connection, qualifier, resource, context) {
        let resourceManager;
        if (context.resources.has(resource.id)) {
            resourceManager = context.resources.ensure(resource.id);
        }
        else {
            resourceManager = new ResourceManager.Default();
            context.resources.set(resource.id, resourceManager);
        }
        const clazz = class extends Resource.Default {
            _rep;
            static async $new(...args) {
                const ctor = resource.getCallable('constructor');
                const result = await ctor.callWasmConstructorAsync(connection, qualifier, args, context);
                return new clazz(ResourceManager.handleTag, result, resourceManager.getRepresentation(result));
            }
            constructor(_handleTag, handle, rep) {
                super(handle);
                this._rep = rep;
            }
            $rep() {
                return this._rep;
            }
        };
        for (const [name, callable] of resource.callables) {
            if (callable instanceof MethodType) {
                clazz.prototype[name] = function (...params) {
                    return callable.callWorkerMethod(connection, qualifier, this, params, context);
                };
            }
            else if (callable instanceof DestructorType) {
                clazz.prototype[name] = function (...params) {
                    return callable.callWorkerMethod(connection, qualifier, this, params, context);
                };
            }
            else if (callable instanceof StaticMethodType) {
                clazz[name] = (...params) => {
                    return callable.callWorker(connection, qualifier, params, context);
                };
            }
        }
        return clazz;
    }
    clazz_1.createPromise = createPromise;
})(clazz || (clazz = {}));
var $main;
(function ($main) {
    function bind(world, service, code, portOrContext, context) {
        if (portOrContext === undefined) {
            return bindSync(world, service, code, new WasmContext.Default());
        }
        else if (ComponentModelContext.is(portOrContext)) {
            return bindSync(world, service, code, portOrContext);
        }
        else {
            return bindAsync(world, service, code, portOrContext, context ?? { options: { encoding: 'utf-8' }, resources: new ResourceManagers.Default() });
        }
    }
    $main.bind = bind;
    async function bindSync(world, service, code, context) {
        const wasmContext = context !== undefined ? new WasmContext.Default(context.options, context.resources) : new WasmContext.Default();
        let module;
        let memory = undefined;
        if (code.module !== undefined) {
            module = code.module;
            memory = code.memory;
        }
        else {
            module = code;
        }
        const imports = $imports.create(world, service, wasmContext);
        if (memory !== undefined) {
            imports.env.memory = memory;
        }
        const instance = await (0, ral_1.default)().WebAssembly.instantiate(module, imports);
        wasmContext.initialize(new Memory.Default(instance.exports));
        return $exports.bind(world, instance.exports, wasmContext);
    }
    async function bindAsync(world, service, code, port, context) {
        const connection = await (0, ral_1.default)().Connection.createMain(port);
        connection.listen();
        await connection.initialize(code, context.options);
        bindServiceAsync(connection, world, service, context);
        return bindApi(connection, world, context);
    }
    function bindServiceAsync(connection, world, service, context) {
        const packageName = world.id.substring(0, world.id.indexOf('/'));
        if (world.imports !== undefined) {
            if (world.imports.functions !== undefined) {
                doBindServiceAsync(connection, '$root', world.imports.functions, undefined, service, context);
            }
            if (world.imports.interfaces !== undefined) {
                for (const [name, iface] of world.imports.interfaces) {
                    const propName = `${name[0].toLowerCase()}${name.substring(1)}`;
                    doBindServiceAsync(connection, `${packageName}/${iface.witName}`, iface.functions, iface.resources, service[propName], context);
                }
            }
        }
        if (world.exports !== undefined) {
            if (world.exports.interfaces !== undefined) {
                for (const iface of world.exports.interfaces.values()) {
                    if (iface.resources === undefined) {
                        continue;
                    }
                    const qualifier = `[export]${packageName}/${iface.witName}`;
                    for (const resource of iface.resources.values()) {
                        const manager = getResourceManager(resource, undefined, context);
                        connection.on(`${qualifier}#[resource-new]${resource.witName}`, (_memory, params) => manager.newHandle(params[0]));
                        connection.on(`${qualifier}#[resource-rep]${resource.witName}`, (_memory, params) => manager.getRepresentation(params[0]));
                        connection.on(`${qualifier}#[resource-drop]${resource.witName}`, (_memory, params) => manager.dropHandle(params[0]));
                    }
                }
            }
        }
    }
    function doBindServiceAsync(connection, qualifier, functions, resources, service, context) {
        if (functions !== undefined) {
            for (const [funcName, func] of functions) {
                connection.on(`${qualifier}#${func.witName}`, (memory, params) => {
                    return func.callServiceAsync(memory, service[funcName], params, context);
                });
            }
        }
        if (resources !== undefined) {
            for (const [resourceName, resource] of resources) {
                const clazz = service[resourceName];
                const resourceManager = getResourceManager(resource, clazz, context);
                for (const [callableName, callable] of resource.callables) {
                    if (callable instanceof ConstructorType) {
                        connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                            return callable.callServiceAsync(memory, clazz, params, context);
                        });
                    }
                    else if (callable instanceof StaticMethodType) {
                        connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                            return callable.callServiceAsync(memory, service[resourceName][callableName], params, context);
                        });
                    }
                    else if (callable instanceof MethodType) {
                        connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                            return callable.callServiceAsync(memory, callableName, params, resourceManager, context);
                        });
                    }
                    else if (callable instanceof DestructorType) {
                        connection.on(`${qualifier}#${callable.witName}`, (memory, params) => {
                            return callable.callServiceAsync(memory, params, resourceManager);
                        });
                    }
                }
            }
        }
    }
    function bindApi(connection, world, context) {
        const packageName = world.id.substring(0, world.id.indexOf('/'));
        const result = Object.create(null);
        if (world.exports !== undefined) {
            if (world.exports.functions !== undefined) {
                Object.assign(result, doBindApi(connection, packageName, world.exports.functions, undefined, context));
            }
            if (world.exports.interfaces !== undefined) {
                for (const [name, iface] of world.exports.interfaces) {
                    const propName = `${name[0].toLowerCase()}${name.substring(1)}`;
                    result[propName] = doBindApi(connection, `${packageName}/${iface.witName}`, iface.functions, iface.resources, context);
                }
            }
        }
        return result;
    }
    function doBindApi(connection, qualifier, functions, resources, context) {
        const result = Object.create(null);
        if (functions !== undefined) {
            for (const [name, func] of functions) {
                result[name] = (...params) => {
                    return func.callWorker(connection, qualifier, params, context);
                };
            }
        }
        if (resources !== undefined) {
            for (const [name, resource] of resources) {
                const resourceManager = getResourceManager(resource, undefined, context);
                const cl = clazz.createPromise(connection, qualifier, resource, context);
                resourceManager.setProxyInfo(cl, (self) => {
                    connection.callWorker(`${qualifier}#[dtor]${resource.witName}`, [self]).
                        catch(() => {
                        (0, ral_1.default)().console.error(`Failed to call destructor for ${resource.witName}`);
                    });
                });
                result[name] = cl;
            }
        }
        return result;
    }
})($main || (exports.$main = $main = {}));


/***/ }),
/* 9 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   NIL: () => (/* reexport safe */ _nil_js__WEBPACK_IMPORTED_MODULE_4__["default"]),
/* harmony export */   parse: () => (/* reexport safe */ _parse_js__WEBPACK_IMPORTED_MODULE_8__["default"]),
/* harmony export */   stringify: () => (/* reexport safe */ _stringify_js__WEBPACK_IMPORTED_MODULE_7__["default"]),
/* harmony export */   v1: () => (/* reexport safe */ _v1_js__WEBPACK_IMPORTED_MODULE_0__["default"]),
/* harmony export */   v3: () => (/* reexport safe */ _v3_js__WEBPACK_IMPORTED_MODULE_1__["default"]),
/* harmony export */   v4: () => (/* reexport safe */ _v4_js__WEBPACK_IMPORTED_MODULE_2__["default"]),
/* harmony export */   v5: () => (/* reexport safe */ _v5_js__WEBPACK_IMPORTED_MODULE_3__["default"]),
/* harmony export */   validate: () => (/* reexport safe */ _validate_js__WEBPACK_IMPORTED_MODULE_6__["default"]),
/* harmony export */   version: () => (/* reexport safe */ _version_js__WEBPACK_IMPORTED_MODULE_5__["default"])
/* harmony export */ });
/* harmony import */ var _v1_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(10);
/* harmony import */ var _v3_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(16);
/* harmony import */ var _v4_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(20);
/* harmony import */ var _v5_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(22);
/* harmony import */ var _nil_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(24);
/* harmony import */ var _version_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(25);
/* harmony import */ var _validate_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(14);
/* harmony import */ var _stringify_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(13);
/* harmony import */ var _parse_js__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(18);










/***/ }),
/* 10 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _rng_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(11);
/* harmony import */ var _stringify_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(13);

 // **`v1()` - Generate time-based UUID**
//
// Inspired by https://github.com/LiosK/UUID.js
// and http://docs.python.org/library/uuid.html

let _nodeId;

let _clockseq; // Previous uuid creation time


let _lastMSecs = 0;
let _lastNSecs = 0; // See https://github.com/uuidjs/uuid for API details

function v1(options, buf, offset) {
  let i = buf && offset || 0;
  const b = buf || new Array(16);
  options = options || {};
  let node = options.node || _nodeId;
  let clockseq = options.clockseq !== undefined ? options.clockseq : _clockseq; // node and clockseq need to be initialized to random values if they're not
  // specified.  We do this lazily to minimize issues related to insufficient
  // system entropy.  See #189

  if (node == null || clockseq == null) {
    const seedBytes = options.random || (options.rng || _rng_js__WEBPACK_IMPORTED_MODULE_0__["default"])();

    if (node == null) {
      // Per 4.5, create and 48-bit node id, (47 random bits + multicast bit = 1)
      node = _nodeId = [seedBytes[0] | 0x01, seedBytes[1], seedBytes[2], seedBytes[3], seedBytes[4], seedBytes[5]];
    }

    if (clockseq == null) {
      // Per 4.2.2, randomize (14 bit) clockseq
      clockseq = _clockseq = (seedBytes[6] << 8 | seedBytes[7]) & 0x3fff;
    }
  } // UUID timestamps are 100 nano-second units since the Gregorian epoch,
  // (1582-10-15 00:00).  JSNumbers aren't precise enough for this, so
  // time is handled internally as 'msecs' (integer milliseconds) and 'nsecs'
  // (100-nanoseconds offset from msecs) since unix epoch, 1970-01-01 00:00.


  let msecs = options.msecs !== undefined ? options.msecs : Date.now(); // Per 4.2.1.2, use count of uuid's generated during the current clock
  // cycle to simulate higher resolution clock

  let nsecs = options.nsecs !== undefined ? options.nsecs : _lastNSecs + 1; // Time since last uuid creation (in msecs)

  const dt = msecs - _lastMSecs + (nsecs - _lastNSecs) / 10000; // Per 4.2.1.2, Bump clockseq on clock regression

  if (dt < 0 && options.clockseq === undefined) {
    clockseq = clockseq + 1 & 0x3fff;
  } // Reset nsecs if clock regresses (new clockseq) or we've moved onto a new
  // time interval


  if ((dt < 0 || msecs > _lastMSecs) && options.nsecs === undefined) {
    nsecs = 0;
  } // Per 4.2.1.2 Throw error if too many uuids are requested


  if (nsecs >= 10000) {
    throw new Error("uuid.v1(): Can't create more than 10M uuids/sec");
  }

  _lastMSecs = msecs;
  _lastNSecs = nsecs;
  _clockseq = clockseq; // Per 4.1.4 - Convert from unix epoch to Gregorian epoch

  msecs += 12219292800000; // `time_low`

  const tl = ((msecs & 0xfffffff) * 10000 + nsecs) % 0x100000000;
  b[i++] = tl >>> 24 & 0xff;
  b[i++] = tl >>> 16 & 0xff;
  b[i++] = tl >>> 8 & 0xff;
  b[i++] = tl & 0xff; // `time_mid`

  const tmh = msecs / 0x100000000 * 10000 & 0xfffffff;
  b[i++] = tmh >>> 8 & 0xff;
  b[i++] = tmh & 0xff; // `time_high_and_version`

  b[i++] = tmh >>> 24 & 0xf | 0x10; // include version

  b[i++] = tmh >>> 16 & 0xff; // `clock_seq_hi_and_reserved` (Per 4.2.2 - include variant)

  b[i++] = clockseq >>> 8 | 0x80; // `clock_seq_low`

  b[i++] = clockseq & 0xff; // `node`

  for (let n = 0; n < 6; ++n) {
    b[i + n] = node[n];
  }

  return buf || (0,_stringify_js__WEBPACK_IMPORTED_MODULE_1__.unsafeStringify)(b);
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (v1);

/***/ }),
/* 11 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (/* binding */ rng)
/* harmony export */ });
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(12);
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(crypto__WEBPACK_IMPORTED_MODULE_0__);

const rnds8Pool = new Uint8Array(256); // # of random values to pre-allocate

let poolPtr = rnds8Pool.length;
function rng() {
  if (poolPtr > rnds8Pool.length - 16) {
    crypto__WEBPACK_IMPORTED_MODULE_0___default().randomFillSync(rnds8Pool);
    poolPtr = 0;
  }

  return rnds8Pool.slice(poolPtr, poolPtr += 16);
}

/***/ }),
/* 12 */
/***/ ((module) => {

module.exports = require("crypto");

/***/ }),
/* 13 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__),
/* harmony export */   unsafeStringify: () => (/* binding */ unsafeStringify)
/* harmony export */ });
/* harmony import */ var _validate_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(14);

/**
 * Convert array of 16 byte values to UUID string format of the form:
 * XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
 */

const byteToHex = [];

for (let i = 0; i < 256; ++i) {
  byteToHex.push((i + 0x100).toString(16).slice(1));
}

function unsafeStringify(arr, offset = 0) {
  // Note: Be careful editing this code!  It's been tuned for performance
  // and works in ways you may not expect. See https://github.com/uuidjs/uuid/pull/434
  return byteToHex[arr[offset + 0]] + byteToHex[arr[offset + 1]] + byteToHex[arr[offset + 2]] + byteToHex[arr[offset + 3]] + '-' + byteToHex[arr[offset + 4]] + byteToHex[arr[offset + 5]] + '-' + byteToHex[arr[offset + 6]] + byteToHex[arr[offset + 7]] + '-' + byteToHex[arr[offset + 8]] + byteToHex[arr[offset + 9]] + '-' + byteToHex[arr[offset + 10]] + byteToHex[arr[offset + 11]] + byteToHex[arr[offset + 12]] + byteToHex[arr[offset + 13]] + byteToHex[arr[offset + 14]] + byteToHex[arr[offset + 15]];
}

function stringify(arr, offset = 0) {
  const uuid = unsafeStringify(arr, offset); // Consistency check for valid UUID.  If this throws, it's likely due to one
  // of the following:
  // - One or more input array values don't map to a hex octet (leading to
  // "undefined" in the uuid)
  // - Invalid input values for the RFC `version` or `variant` fields

  if (!(0,_validate_js__WEBPACK_IMPORTED_MODULE_0__["default"])(uuid)) {
    throw TypeError('Stringified UUID is invalid');
  }

  return uuid;
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (stringify);

/***/ }),
/* 14 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _regex_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(15);


function validate(uuid) {
  return typeof uuid === 'string' && _regex_js__WEBPACK_IMPORTED_MODULE_0__["default"].test(uuid);
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (validate);

/***/ }),
/* 15 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (/^(?:[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}|00000000-0000-0000-0000-000000000000)$/i);

/***/ }),
/* 16 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _v35_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(17);
/* harmony import */ var _md5_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(19);


const v3 = (0,_v35_js__WEBPACK_IMPORTED_MODULE_0__["default"])('v3', 0x30, _md5_js__WEBPACK_IMPORTED_MODULE_1__["default"]);
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (v3);

/***/ }),
/* 17 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   DNS: () => (/* binding */ DNS),
/* harmony export */   URL: () => (/* binding */ URL),
/* harmony export */   "default": () => (/* binding */ v35)
/* harmony export */ });
/* harmony import */ var _stringify_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(13);
/* harmony import */ var _parse_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(18);



function stringToBytes(str) {
  str = unescape(encodeURIComponent(str)); // UTF8 escape

  const bytes = [];

  for (let i = 0; i < str.length; ++i) {
    bytes.push(str.charCodeAt(i));
  }

  return bytes;
}

const DNS = '6ba7b810-9dad-11d1-80b4-00c04fd430c8';
const URL = '6ba7b811-9dad-11d1-80b4-00c04fd430c8';
function v35(name, version, hashfunc) {
  function generateUUID(value, namespace, buf, offset) {
    var _namespace;

    if (typeof value === 'string') {
      value = stringToBytes(value);
    }

    if (typeof namespace === 'string') {
      namespace = (0,_parse_js__WEBPACK_IMPORTED_MODULE_0__["default"])(namespace);
    }

    if (((_namespace = namespace) === null || _namespace === void 0 ? void 0 : _namespace.length) !== 16) {
      throw TypeError('Namespace must be array-like (16 iterable integer values, 0-255)');
    } // Compute hash of namespace and value, Per 4.3
    // Future: Use spread syntax when supported on all platforms, e.g. `bytes =
    // hashfunc([...namespace, ... value])`


    let bytes = new Uint8Array(16 + value.length);
    bytes.set(namespace);
    bytes.set(value, namespace.length);
    bytes = hashfunc(bytes);
    bytes[6] = bytes[6] & 0x0f | version;
    bytes[8] = bytes[8] & 0x3f | 0x80;

    if (buf) {
      offset = offset || 0;

      for (let i = 0; i < 16; ++i) {
        buf[offset + i] = bytes[i];
      }

      return buf;
    }

    return (0,_stringify_js__WEBPACK_IMPORTED_MODULE_1__.unsafeStringify)(bytes);
  } // Function#name is not settable on some platforms (#270)


  try {
    generateUUID.name = name; // eslint-disable-next-line no-empty
  } catch (err) {} // For CommonJS default export support


  generateUUID.DNS = DNS;
  generateUUID.URL = URL;
  return generateUUID;
}

/***/ }),
/* 18 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _validate_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(14);


function parse(uuid) {
  if (!(0,_validate_js__WEBPACK_IMPORTED_MODULE_0__["default"])(uuid)) {
    throw TypeError('Invalid UUID');
  }

  let v;
  const arr = new Uint8Array(16); // Parse ########-....-....-....-............

  arr[0] = (v = parseInt(uuid.slice(0, 8), 16)) >>> 24;
  arr[1] = v >>> 16 & 0xff;
  arr[2] = v >>> 8 & 0xff;
  arr[3] = v & 0xff; // Parse ........-####-....-....-............

  arr[4] = (v = parseInt(uuid.slice(9, 13), 16)) >>> 8;
  arr[5] = v & 0xff; // Parse ........-....-####-....-............

  arr[6] = (v = parseInt(uuid.slice(14, 18), 16)) >>> 8;
  arr[7] = v & 0xff; // Parse ........-....-....-####-............

  arr[8] = (v = parseInt(uuid.slice(19, 23), 16)) >>> 8;
  arr[9] = v & 0xff; // Parse ........-....-....-....-############
  // (Use "/" to avoid 32-bit truncation when bit-shifting high-order bytes)

  arr[10] = (v = parseInt(uuid.slice(24, 36), 16)) / 0x10000000000 & 0xff;
  arr[11] = v / 0x100000000 & 0xff;
  arr[12] = v >>> 24 & 0xff;
  arr[13] = v >>> 16 & 0xff;
  arr[14] = v >>> 8 & 0xff;
  arr[15] = v & 0xff;
  return arr;
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (parse);

/***/ }),
/* 19 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(12);
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(crypto__WEBPACK_IMPORTED_MODULE_0__);


function md5(bytes) {
  if (Array.isArray(bytes)) {
    bytes = Buffer.from(bytes);
  } else if (typeof bytes === 'string') {
    bytes = Buffer.from(bytes, 'utf8');
  }

  return crypto__WEBPACK_IMPORTED_MODULE_0___default().createHash('md5').update(bytes).digest();
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (md5);

/***/ }),
/* 20 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _native_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(21);
/* harmony import */ var _rng_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(11);
/* harmony import */ var _stringify_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(13);




function v4(options, buf, offset) {
  if (_native_js__WEBPACK_IMPORTED_MODULE_0__["default"].randomUUID && !buf && !options) {
    return _native_js__WEBPACK_IMPORTED_MODULE_0__["default"].randomUUID();
  }

  options = options || {};
  const rnds = options.random || (options.rng || _rng_js__WEBPACK_IMPORTED_MODULE_1__["default"])(); // Per 4.4, set bits for version and `clock_seq_hi_and_reserved`

  rnds[6] = rnds[6] & 0x0f | 0x40;
  rnds[8] = rnds[8] & 0x3f | 0x80; // Copy bytes to buffer, if provided

  if (buf) {
    offset = offset || 0;

    for (let i = 0; i < 16; ++i) {
      buf[offset + i] = rnds[i];
    }

    return buf;
  }

  return (0,_stringify_js__WEBPACK_IMPORTED_MODULE_2__.unsafeStringify)(rnds);
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (v4);

/***/ }),
/* 21 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(12);
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(crypto__WEBPACK_IMPORTED_MODULE_0__);

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
  randomUUID: (crypto__WEBPACK_IMPORTED_MODULE_0___default().randomUUID)
});

/***/ }),
/* 22 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _v35_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(17);
/* harmony import */ var _sha1_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(23);


const v5 = (0,_v35_js__WEBPACK_IMPORTED_MODULE_0__["default"])('v5', 0x50, _sha1_js__WEBPACK_IMPORTED_MODULE_1__["default"]);
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (v5);

/***/ }),
/* 23 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(12);
/* harmony import */ var crypto__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(crypto__WEBPACK_IMPORTED_MODULE_0__);


function sha1(bytes) {
  if (Array.isArray(bytes)) {
    bytes = Buffer.from(bytes);
  } else if (typeof bytes === 'string') {
    bytes = Buffer.from(bytes, 'utf8');
  }

  return crypto__WEBPACK_IMPORTED_MODULE_0___default().createHash('sha1').update(bytes).digest();
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (sha1);

/***/ }),
/* 24 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = ('00000000-0000-0000-0000-000000000000');

/***/ }),
/* 25 */
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (__WEBPACK_DEFAULT_EXPORT__)
/* harmony export */ });
/* harmony import */ var _validate_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(14);


function version(uuid) {
  if (!(0,_validate_js__WEBPACK_IMPORTED_MODULE_0__["default"])(uuid)) {
    throw TypeError('Invalid UUID');
  }

  return parseInt(uuid.slice(14, 15), 16);
}

/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (version);

/***/ }),
/* 26 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.BaseMainConnection = exports.BaseWorkerConnection = exports.Connection = void 0;
/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
/// <reference path="../../typings/webAssemblyCommon.d.ts" preserve="true"/>
const uuid = __importStar(__webpack_require__(9));
const componentModel_1 = __webpack_require__(8);
const promises_1 = __webpack_require__(27);
const ral_1 = __importDefault(__webpack_require__(6));
const semaphore_1 = __webpack_require__(28);
class ConnectionMemory {
    static Header = {
        sync: { offset: 0, size: 4 },
        errorCode: { offset: 4, size: 4 },
        resultType: { offset: 8, size: 4 },
        result: { offset: 12, size: 8 },
        next: { offset: 20, size: 4 },
        end: { offset: 24, size: 0 },
    };
    buffer;
    id;
    next;
    constructor(sizeOrBuffer, id) {
        if (sizeOrBuffer === undefined) {
            sizeOrBuffer = 64 * 1024;
        }
        if (typeof sizeOrBuffer === 'number') {
            this.id = uuid.v4();
            this.buffer = new SharedArrayBuffer(sizeOrBuffer);
            this.next = new Uint32Array(this.buffer, ConnectionMemory.Header.next.offset, 1);
            this.next[0] = ConnectionMemory.Header.end.offset;
        }
        else {
            this.buffer = sizeOrBuffer;
            this.id = id;
            this.next = new Uint32Array(this.buffer, ConnectionMemory.Header.next.offset, 1);
        }
    }
    reset() {
        const view = new Uint8Array(this.buffer, 0, ConnectionMemory.Header.end.offset);
        view.fill(0);
        this.next[0] = ConnectionMemory.Header.end.offset;
    }
    alloc(align, size) {
        const next = this.next[0];
        const result = componentModel_1.Alignment.align(next, align);
        this.next[0] = result + size;
        return new componentModel_1.MemoryRange(this, result, size);
    }
    realloc() {
        throw new componentModel_1.ComponentModelTrap('ConnectionMemory does not support realloc');
    }
    preAllocated(ptr, size) {
        return new componentModel_1.MemoryRange(this, ptr, size);
    }
    readonly(ptr, size) {
        return new componentModel_1.ReadonlyMemoryRange(this, ptr, size);
    }
}
class Connection {
    static createWorker(world, port, timeout) {
        return (0, ral_1.default)().Connection.createWorker(port, world, timeout);
    }
    memory;
    constructor(memory) {
        this.memory = memory;
    }
    static serializeParams(params) {
        const result = [];
        for (const param of params) {
            if (typeof param === 'number') {
                result.push(param);
            }
            else {
                result.push(param.toString());
            }
        }
        return result;
    }
    static deserializeParams(params) {
        const result = [];
        for (const param of params) {
            if (typeof param === 'string') {
                result.push(BigInt(param));
            }
            else {
                result.push(param);
            }
        }
        return result;
    }
    static serializeResult(result) {
        if (typeof result === 'bigint') {
            return result.toString();
        }
        else if (typeof result === 'number') {
            return result;
        }
        return undefined;
    }
    static deserializeResult(result) {
        if (result === undefined) {
            return;
        }
        return typeof result === 'number' ? result : BigInt(result);
    }
    static loadResult(buffer) {
        const view = new DataView(buffer, 0, ConnectionMemory.Header.end.offset);
        const resultType = view.getUint32(ConnectionMemory.Header.resultType.offset, true);
        switch (resultType) {
            case Connection.WasmTypeKind.undefined:
                return;
            case Connection.WasmTypeKind.float:
                return view.getFloat64(ConnectionMemory.Header.result.offset, true);
            case Connection.WasmTypeKind.signed:
                return view.getBigInt64(ConnectionMemory.Header.result.offset, true);
            case Connection.WasmTypeKind.unsigned:
                return view.getBigUint64(ConnectionMemory.Header.result.offset, true);
            default:
                throw new componentModel_1.ComponentModelTrap(`Unexpected result type ${resultType}`);
        }
    }
    static storeResult(buffer, result) {
        const view = new DataView(buffer, 0, ConnectionMemory.Header.end.offset);
        if (result === undefined) {
            view.setUint32(ConnectionMemory.Header.resultType.offset, Connection.WasmTypeKind.undefined, true);
        }
        else if (typeof result === 'bigint') {
            if (result < 0) {
                view.setUint32(ConnectionMemory.Header.resultType.offset, Connection.WasmTypeKind.signed, true);
                view.setBigInt64(ConnectionMemory.Header.result.offset, result, true);
            }
            else {
                view.setUint32(ConnectionMemory.Header.resultType.offset, Connection.WasmTypeKind.unsigned, true);
                view.setBigUint64(ConnectionMemory.Header.result.offset, result, true);
            }
        }
        else if (typeof result === 'number') {
            view.setUint32(ConnectionMemory.Header.resultType.offset, Connection.WasmTypeKind.float, true);
            view.setFloat64(ConnectionMemory.Header.result.offset, result, true);
        }
        else {
            throw new componentModel_1.ComponentModelTrap(`Unexpected result type ${result}`);
        }
    }
}
exports.Connection = Connection;
(function (Connection) {
    let ErrorCodes;
    (function (ErrorCodes) {
        ErrorCodes[ErrorCodes["noHandler"] = 1] = "noHandler";
        ErrorCodes[ErrorCodes["promiseRejected"] = 2] = "promiseRejected";
    })(ErrorCodes = Connection.ErrorCodes || (Connection.ErrorCodes = {}));
    let WasmTypeKind;
    (function (WasmTypeKind) {
        WasmTypeKind[WasmTypeKind["undefined"] = 0] = "undefined";
        WasmTypeKind[WasmTypeKind["float"] = 1] = "float";
        WasmTypeKind[WasmTypeKind["signed"] = 2] = "signed";
        WasmTypeKind[WasmTypeKind["unsigned"] = 3] = "unsigned";
    })(WasmTypeKind = Connection.WasmTypeKind || (Connection.WasmTypeKind = {}));
})(Connection || (exports.Connection = Connection = {}));
class BaseWorkerConnection extends Connection {
    world;
    timeout;
    handlers;
    constructor(world, timeout) {
        super(new ConnectionMemory());
        this.world = world;
        this.timeout = timeout;
        this.handlers = new Map();
    }
    dispose() {
        this.handlers.clear();
    }
    on(name, handler) {
        this.handlers.set(name, handler);
    }
    getMemory() {
        return this.memory;
    }
    prepareCall() {
        this.memory.reset();
    }
    callMain(name, params) {
        const buffer = this.memory.buffer;
        const sync = new Int32Array(buffer, ConnectionMemory.Header.sync.offset, 1);
        Atomics.store(sync, 0, 0);
        const message = {
            method: 'callMain',
            name: name,
            params: Connection.serializeParams(params),
            memory: { buffer: this.memory.buffer, id: this.memory.id }
        };
        this.postMessage(message);
        // Wait for the answer
        const result = Atomics.wait(sync, 0, 0, this.timeout);
        switch (result) {
            case 'timed-out':
                throw new componentModel_1.ComponentModelTrap(`Call ${name} to main thread timed out`);
            case 'not-equal':
                const value = Atomics.load(sync, 0);
                // If the value === 1 the service has already provided the result.
                // Otherwise we actually don't know what happened :-(.
                if (value !== 1) {
                    throw new componentModel_1.ComponentModelTrap(`Unexpected value ${value} in sync object`);
                }
        }
        return Connection.loadResult(buffer);
    }
    handleMessage(message) {
        if (message.method === 'initializeWorker') {
            const wasmContext = new componentModel_1.WasmContext.Default(message.options);
            const imports = componentModel_1.$imports.worker.create(this, this.world, wasmContext);
            if (message.memory !== undefined) {
                imports.env.memory = message.memory;
            }
            (0, ral_1.default)().WebAssembly.instantiate(message.module, imports).then((instance) => {
                wasmContext.initialize(new componentModel_1.Memory.Default(instance.exports));
                componentModel_1.$exports.worker.bind(this, this.world, instance.exports, wasmContext);
                this.postMessage({ method: 'reportResult', name: '$initializeWorker', result: 'success' });
            }).catch((error) => {
                this.postMessage({ method: 'reportResult', name: '$initializeWorker', error: error.toString() });
            });
        }
        else if (message.method === 'callWorker') {
            const handler = this.handlers.get(message.name);
            if (handler === undefined) {
                this.postMessage({ method: 'reportResult', name: message.name, error: `No handler found for ${message.name}` });
                return;
            }
            try {
                const memory = new ConnectionMemory(message.memory.buffer, message.memory.id);
                const result = handler(memory, Connection.deserializeParams(message.params));
                this.postMessage({ method: 'reportResult', name: message.name, result: Connection.serializeResult(result) });
            }
            catch (error) {
                this.postMessage({ method: 'reportResult', name: message.name, error: `Calling WASM function ${message.name} failed.` });
            }
        }
    }
}
exports.BaseWorkerConnection = BaseWorkerConnection;
class BaseMainConnection extends Connection {
    initializeCall;
    handlers;
    callQueue;
    currentCall;
    constructor() {
        super(new ConnectionMemory());
        this.handlers = new Map();
        this.callQueue = new semaphore_1.Semaphore(1);
        this.currentCall = undefined;
    }
    dispose() {
        this.handlers.clear();
        this.callQueue.dispose();
    }
    lock(thunk) {
        return this.callQueue.lock(thunk);
    }
    prepareCall() {
        this.memory.reset();
    }
    getMemory() {
        return this.memory;
    }
    async initialize(code, options) {
        let module;
        let memory = undefined;
        if (code.module !== undefined) {
            module = code.module;
            memory = code.memory;
        }
        else {
            module = code;
        }
        return new Promise((resolve, reject) => {
            const message = {
                method: 'initializeWorker',
                module: module,
                memory: memory,
                options: options,
            };
            this.initializeCall = { resolve, reject };
            this.postMessage(message);
        });
    }
    callWorker(name, params) {
        if (this.currentCall !== undefined) {
            throw new componentModel_1.ComponentModelTrap('Call already in progress');
        }
        this.currentCall = promises_1.CapturedPromise.create();
        const message = {
            method: 'callWorker',
            name: name,
            params: Connection.serializeParams(params),
            memory: { buffer: this.memory.buffer, id: this.memory.id }
        };
        this.postMessage(message);
        return this.currentCall.promise;
    }
    on(id, handler) {
        this.handlers.set(id, handler);
    }
    handleMessage(message) {
        if (message.method === 'callMain') {
            const buffer = message.memory.buffer;
            const sync = new Int32Array(buffer, ConnectionMemory.Header.sync.offset, 1);
            const view = new DataView(buffer, 0, ConnectionMemory.Header.end.offset);
            const handler = this.handlers.get(message.name);
            if (handler === undefined) {
                view.setUint32(ConnectionMemory.Header.errorCode.offset, Connection.ErrorCodes.noHandler, true);
                Atomics.store(sync, 0, 1);
                Atomics.notify(sync, 0);
                return;
            }
            else {
                const memory = new ConnectionMemory(buffer, message.memory.id);
                const params = Connection.deserializeParams(message.params);
                const result = handler(memory, params);
                if (result instanceof Promise) {
                    result.then((value) => {
                        Connection.storeResult(buffer, value);
                    }).catch(() => {
                        view.setUint32(ConnectionMemory.Header.errorCode.offset, Connection.ErrorCodes.promiseRejected, true);
                    }).finally(() => {
                        Atomics.store(sync, 0, 1);
                        Atomics.notify(sync, 0);
                    });
                }
                else {
                    Connection.storeResult(buffer, result);
                    Atomics.store(sync, 0, 1);
                    Atomics.notify(sync, 0);
                }
            }
        }
        else if (message.method === 'reportResult') {
            if (message.name === '$initializeWorker') {
                if (this.initializeCall === undefined) {
                    // Need to think about logging this.
                    return;
                }
                if (message.error !== undefined) {
                    this.initializeCall.reject(new Error(message.error));
                }
                else {
                    this.initializeCall.resolve();
                }
                this.initializeCall = undefined;
            }
            else {
                if (this.currentCall === undefined) {
                    // Need to think about logging this.
                    return;
                }
                if (message.error !== undefined) {
                    this.currentCall.reject(new Error(message.error));
                }
                else {
                    const result = Connection.deserializeResult(message.result);
                    this.currentCall.resolve(result);
                }
                this.currentCall = undefined;
            }
        }
    }
}
exports.BaseMainConnection = BaseMainConnection;


/***/ }),
/* 27 */
/***/ ((__unused_webpack_module, exports) => {


/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.CapturedPromise = void 0;
var CapturedPromise;
(function (CapturedPromise) {
    function create() {
        let _resolve;
        let _reject;
        const promise = new Promise((resolve, reject) => {
            _resolve = resolve;
            _reject = reject;
        });
        return {
            promise, resolve: _resolve, reject: _reject
        };
    }
    CapturedPromise.create = create;
})(CapturedPromise || (exports.CapturedPromise = CapturedPromise = {}));


/***/ }),
/* 28 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.Semaphore = void 0;
const ral_1 = __importDefault(__webpack_require__(6));
class Semaphore {
    _capacity;
    _active;
    _waiting;
    constructor(capacity = 1) {
        if (capacity <= 0) {
            throw new Error('Capacity must be greater than 0');
        }
        this._capacity = capacity;
        this._active = 0;
        this._waiting = [];
    }
    dispose() {
        for (const item of this._waiting) {
            item.reject(new Error('Semaphore disposed'));
        }
        this._active = 0;
        this._waiting = [];
    }
    lock(thunk) {
        return new Promise((resolve, reject) => {
            this._waiting.push({ thunk, resolve, reject });
            this.runNext();
        });
    }
    get active() {
        return this._active;
    }
    runNext() {
        if (this._waiting.length === 0 || this._active === this._capacity) {
            return;
        }
        (0, ral_1.default)().timer.setImmediate(() => this.doRunNext());
    }
    doRunNext() {
        if (this._waiting.length === 0 || this._active === this._capacity) {
            return;
        }
        const next = this._waiting.shift();
        this._active++;
        if (this._active > this._capacity) {
            throw new Error(`To many thunks active`);
        }
        try {
            const result = next.thunk();
            if (result instanceof Promise) {
                result.then((value) => {
                    this._active--;
                    next.resolve(value);
                    this.runNext();
                }, (err) => {
                    this._active--;
                    next.reject(err);
                    this.runNext();
                });
            }
            else {
                this._active--;
                next.resolve(result);
                this.runNext();
            }
        }
        catch (err) {
            this._active--;
            next.reject(err);
            this.runNext();
        }
    }
}
exports.Semaphore = Semaphore;


/***/ }),
/* 29 */,
/* 30 */
/***/ (function(__unused_webpack_module, exports, __webpack_require__) {


var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", ({ value: true }));
exports.tergo = void 0;
/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
/* eslint-disable @typescript-eslint/ban-types */
const $wcm = __importStar(__webpack_require__(2));
var tergo;
(function (tergo) {
    var $;
    (function ($) {
        let imports;
        (function (imports) {
            imports.log = new $wcm.FunctionType('log', [
                ['msg', $wcm.wstring],
            ], undefined);
        })(imports = $.imports || ($.imports = {}));
        let exports;
        (function (exports) {
            exports.format = new $wcm.FunctionType('format', [
                ['code', $wcm.wstring],
            ], new $wcm.ResultType($wcm.wstring, $wcm.wstring, $wcm.wstring.Error));
        })(exports = $.exports || ($.exports = {}));
    })($ = tergo.$ || (tergo.$ = {}));
})(tergo || (exports.tergo = tergo = {}));
(function (tergo) {
    var _;
    (function (_) {
        _.id = 'vscode:tergo/tergo';
        _.witName = 'tergo';
        let imports;
        (function (imports) {
            imports.functions = new Map([
                ['log', tergo.$.imports.log]
            ]);
            function create(service, context) {
                return $wcm.$imports.create(_, service, context);
            }
            imports.create = create;
            function loop(service, context) {
                return $wcm.$imports.loop(_, service, context);
            }
            imports.loop = loop;
        })(imports = _.imports || (_.imports = {}));
        let exports;
        (function (exports_1) {
            exports_1.functions = new Map([
                ['format', tergo.$.exports.format]
            ]);
            function bind(exports, context) {
                return $wcm.$exports.bind(_, exports, context);
            }
            exports_1.bind = bind;
        })(exports = _.exports || (_.exports = {}));
        function bind(service, code, portOrContext, context) {
            return $wcm.$main.bind(_, service, code, portOrContext, context);
        }
        _.bind = bind;
    })(_ = tergo._ || (tergo._ = {}));
})(tergo || (exports.tergo = tergo = {}));


/***/ })
/******/ 	]);
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/compat get default export */
/******/ 	(() => {
/******/ 		// getDefaultExport function for compatibility with non-harmony modules
/******/ 		__webpack_require__.n = (module) => {
/******/ 			var getter = module && module.__esModule ?
/******/ 				() => (module['default']) :
/******/ 				() => (module);
/******/ 			__webpack_require__.d(getter, { a: getter });
/******/ 			return getter;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".extension.js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/require chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded chunks
/******/ 		// "1" means "loaded", otherwise not loaded yet
/******/ 		var installedChunks = {
/******/ 			0: 1
/******/ 		};
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		var installChunk = (chunk) => {
/******/ 			var moreModules = chunk.modules, chunkIds = chunk.ids, runtime = chunk.runtime;
/******/ 			for(var moduleId in moreModules) {
/******/ 				if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 					__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 				}
/******/ 			}
/******/ 			if(runtime) runtime(__webpack_require__);
/******/ 			for(var i = 0; i < chunkIds.length; i++)
/******/ 				installedChunks[chunkIds[i]] = 1;
/******/ 		
/******/ 		};
/******/ 		
/******/ 		// require() chunk loading for javascript
/******/ 		__webpack_require__.f.require = (chunkId, promises) => {
/******/ 			// "1" is the signal for "already loaded"
/******/ 			if(!installedChunks[chunkId]) {
/******/ 				if(true) { // all chunks have JS
/******/ 					installChunk(require("./" + __webpack_require__.u(chunkId)));
/******/ 				} else installedChunks[chunkId] = 1;
/******/ 			}
/******/ 		};
/******/ 		
/******/ 		// no external install chunk
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module is referenced by other modules so it can't be inlined
/******/ 	var __webpack_exports__ = __webpack_require__(0);
/******/ 	module.exports = __webpack_exports__;
/******/ 	
/******/ })()
;
//# sourceMappingURL=extension.js.map