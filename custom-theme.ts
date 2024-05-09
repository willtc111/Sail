
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const customTheme: CustomThemeConfig = {
    name: 'custom-theme',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace`,
		"--theme-font-family-heading": `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "2px",
		"--theme-rounded-container": "4px",
		"--theme-border-base": "1px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "255 255 255",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "0 0 0",
		"--on-success": "0 0 0",
		"--on-warning": "0 0 0",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #007a66 
		"--color-primary-50": "217 235 232", // #d9ebe8
		"--color-primary-100": "204 228 224", // #cce4e0
		"--color-primary-200": "191 222 217", // #bfded9
		"--color-primary-300": "153 202 194", // #99cac2
		"--color-primary-400": "77 162 148", // #4da294
		"--color-primary-500": "0 122 102", // #007a66
		"--color-primary-600": "0 110 92", // #006e5c
		"--color-primary-700": "0 92 77", // #005c4d
		"--color-primary-800": "0 73 61", // #00493d
		"--color-primary-900": "0 60 50", // #003c32
		// secondary | #581fad 
		"--color-secondary-50": "230 221 243", // #e6ddf3
		"--color-secondary-100": "222 210 239", // #ded2ef
		"--color-secondary-200": "213 199 235", // #d5c7eb
		"--color-secondary-300": "188 165 222", // #bca5de
		"--color-secondary-400": "138 98 198", // #8a62c6
		"--color-secondary-500": "88 31 173", // #581fad
		"--color-secondary-600": "79 28 156", // #4f1c9c
		"--color-secondary-700": "66 23 130", // #421782
		"--color-secondary-800": "53 19 104", // #351368
		"--color-secondary-900": "43 15 85", // #2b0f55
		// tertiary | #c7b600 
		"--color-tertiary-50": "247 244 217", // #f7f4d9
		"--color-tertiary-100": "244 240 204", // #f4f0cc
		"--color-tertiary-200": "241 237 191", // #f1edbf
		"--color-tertiary-300": "233 226 153", // #e9e299
		"--color-tertiary-400": "216 204 77", // #d8cc4d
		"--color-tertiary-500": "199 182 0", // #c7b600
		"--color-tertiary-600": "179 164 0", // #b3a400
		"--color-tertiary-700": "149 137 0", // #958900
		"--color-tertiary-800": "119 109 0", // #776d00
		"--color-tertiary-900": "98 89 0", // #625900
		// success | #34c200 
		"--color-success-50": "225 246 217", // #e1f6d9
		"--color-success-100": "214 243 204", // #d6f3cc
		"--color-success-200": "204 240 191", // #ccf0bf
		"--color-success-300": "174 231 153", // #aee799
		"--color-success-400": "113 212 77", // #71d44d
		"--color-success-500": "52 194 0", // #34c200
		"--color-success-600": "47 175 0", // #2faf00
		"--color-success-700": "39 146 0", // #279200
		"--color-success-800": "31 116 0", // #1f7400
		"--color-success-900": "25 95 0", // #195f00
		// warning | #ec7d22 
		"--color-warning-50": "252 236 222", // #fcecde
		"--color-warning-100": "251 229 211", // #fbe5d3
		"--color-warning-200": "250 223 200", // #fadfc8
		"--color-warning-300": "247 203 167", // #f7cba7
		"--color-warning-400": "242 164 100", // #f2a464
		"--color-warning-500": "236 125 34", // #ec7d22
		"--color-warning-600": "212 113 31", // #d4711f
		"--color-warning-700": "177 94 26", // #b15e1a
		"--color-warning-800": "142 75 20", // #8e4b14
		"--color-warning-900": "116 61 17", // #743d11
		// error | #d10000 
		"--color-error-50": "248 217 217", // #f8d9d9
		"--color-error-100": "246 204 204", // #f6cccc
		"--color-error-200": "244 191 191", // #f4bfbf
		"--color-error-300": "237 153 153", // #ed9999
		"--color-error-400": "223 77 77", // #df4d4d
		"--color-error-500": "209 0 0", // #d10000
		"--color-error-600": "188 0 0", // #bc0000
		"--color-error-700": "157 0 0", // #9d0000
		"--color-error-800": "125 0 0", // #7d0000
		"--color-error-900": "102 0 0", // #660000
		// surface | #646f7d 
		"--color-surface-50": "232 233 236", // #e8e9ec
		"--color-surface-100": "224 226 229", // #e0e2e5
		"--color-surface-200": "216 219 223", // #d8dbdf
		"--color-surface-300": "193 197 203", // #c1c5cb
		"--color-surface-400": "147 154 164", // #939aa4
		"--color-surface-500": "100 111 125", // #646f7d
		"--color-surface-600": "90 100 113", // #5a6471
		"--color-surface-700": "75 83 94", // #4b535e
		"--color-surface-800": "60 67 75", // #3c434b
		"--color-surface-900": "49 54 61", // #31363d
		
	}
}