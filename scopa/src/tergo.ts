/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
/* eslint-disable @typescript-eslint/ban-types */
import * as $wcm from '@vscode/wasm-component-model';
import type { i32, ptr, result } from '@vscode/wasm-component-model';

export namespace tergo {
	export type Imports = {
		log: (msg: string) => void;
	};
	export namespace Imports {
		export type Promisified = $wcm.$imports.Promisify<Imports>;
	}
	export namespace imports {
		export type Promisify<T> = $wcm.$imports.Promisify<T>;
	}
	export type Exports = {
		/**
		 * @throws $wcm.wstring.Error
		 */
		format: (code: string) => string;
	};
	export namespace Exports {
		export type Promisified = $wcm.$exports.Promisify<Exports>;
	}
	export namespace exports {
		export type Promisify<T> = $wcm.$exports.Promisify<T>;
	}
}

export namespace tergo.$ {
	export namespace imports {
		export const log = new $wcm.FunctionType<tergo.Imports['log']>('log',[
			['msg', $wcm.wstring],
		], undefined);
	}
	export namespace exports {
		export const format = new $wcm.FunctionType<tergo.Exports['format']>('format',[
			['code', $wcm.wstring],
		], new $wcm.ResultType<string, string>($wcm.wstring, $wcm.wstring, $wcm.wstring.Error));
	}
}
export namespace tergo._ {
	export const id = 'vscode:tergo/tergo' as const;
	export const witName = 'tergo' as const;
	export type $Root = {
		'log': (msg_ptr: i32, msg_len: i32) => void;
	};
	export namespace imports {
		export const functions: Map<string, $wcm.FunctionType> = new Map([
			['log', $.imports.log]
		]);
		export function create(service: tergo.Imports, context: $wcm.WasmContext): Imports {
			return $wcm.$imports.create<Imports>(_, service, context);
		}
		export function loop(service: tergo.Imports, context: $wcm.WasmContext): tergo.Imports {
			return $wcm.$imports.loop<tergo.Imports>(_, service, context);
		}
	}
	export type Imports = {
		'$root': $Root;
	};
	export namespace exports {
		export const functions: Map<string, $wcm.FunctionType> = new Map([
			['format', $.exports.format]
		]);
		export function bind(exports: Exports, context: $wcm.WasmContext): tergo.Exports {
			return $wcm.$exports.bind<tergo.Exports>(_, exports, context);
		}
	}
	export type Exports = {
		'format': (code_ptr: i32, code_len: i32, result: ptr<result<string, string>>) => void;
	};
	export function bind(service: tergo.Imports, code: $wcm.Code, context?: $wcm.ComponentModelContext): Promise<tergo.Exports>;
	export function bind(service: tergo.Imports.Promisified, code: $wcm.Code, port: $wcm.RAL.ConnectionPort, context?: $wcm.ComponentModelContext): Promise<tergo.Exports.Promisified>;
	export function bind(service: tergo.Imports | tergo.Imports.Promisified, code: $wcm.Code, portOrContext?: $wcm.RAL.ConnectionPort | $wcm.ComponentModelContext, context?: $wcm.ComponentModelContext | undefined): Promise<tergo.Exports> | Promise<tergo.Exports.Promisified> {
		return $wcm.$main.bind(_, service, code, portOrContext, context);
	}
}