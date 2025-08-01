import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function CircleCoin({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M9.5 1C14.1944 1 18 4.80558 18 9.5C18 11.5861 17.2471 13.4955 16 14.9746V17C16 17.8284 15.3284 18.5 14.5 18.5C13.752 18.5 13.1329 17.9524 13.0195 17.2363C11.9468 17.7251 10.7558 18 9.5 18C4.80558 18 1 14.1944 1 9.5C1 4.80558 4.80558 1 9.5 1Z" fill="url(#1752500502775-6427239_circle-coin_existing_0_9d1ern3py)" mask="url(#1752500502775-6427239_circle-coin_mask_kcbcrf2l5)" data-glass="origin"/>
		<path clipPath="url(#1752500502775-6427239_circle-coin_clipPath_e4v4x5rng)" d="M9.5 1C14.1944 1 18 4.80558 18 9.5C18 11.5861 17.2471 13.4955 16 14.9746V17C16 17.8284 15.3284 18.5 14.5 18.5C13.752 18.5 13.1329 17.9524 13.0195 17.2363C11.9468 17.7251 10.7558 18 9.5 18C4.80558 18 1 14.1944 1 9.5C1 4.80558 4.80558 1 9.5 1Z" fill="url(#1752500502775-6427239_circle-coin_existing_0_9d1ern3py)" data-glass="clone"/>
		<path d="M14.5 6C19.1944 6 23 9.80558 23 14.5C23 19.1944 19.1944 23 14.5 23C9.80558 23 6 19.1944 6 14.5C6 9.80558 9.80558 6 14.5 6ZM14.5 11C13.9477 11 13.5 11.4477 13.5 12V17C13.5 17.5523 13.9477 18 14.5 18C15.0523 18 15.5 17.5523 15.5 17V12C15.5 11.4477 15.0523 11 14.5 11Z" fill="url(#1752500502775-6427239_circle-coin_existing_1_eckaewepf)" data-glass="blur"/>
		<path d="M14.5 6C19.1944 6 23 9.80558 23 14.5C23 19.1944 19.1944 23 14.5 23C9.80558 23 6 19.1944 6 14.5C6 9.80558 9.80558 6 14.5 6ZM14.5 6.75C10.2198 6.75 6.75 10.2198 6.75 14.5C6.75 18.7802 10.2198 22.25 14.5 22.25C18.7802 22.25 22.25 18.7802 22.25 14.5C22.25 10.2198 18.7802 6.75 14.5 6.75Z" fill="url(#1752500502775-6427239_circle-coin_existing_2_hnwplfgmr)"/>
		<defs>
			<linearGradient id="1752500502775-6427239_circle-coin_existing_0_9d1ern3py" gradientUnits="userSpaceOnUse" x1="9.5" x2="9.5" y1="1" y2="18.5">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502775-6427239_circle-coin_existing_1_eckaewepf" gradientUnits="userSpaceOnUse" x1="14.5" x2="14.5" y1="6" y2="23">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502775-6427239_circle-coin_existing_2_hnwplfgmr" gradientUnits="userSpaceOnUse" x1="14.5" x2="14.5" y1="6" y2="15.845">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502775-6427239_circle-coin_clipPath_e4v4x5rng">
				<path d="M14.5 6C19.1944 6 23 9.80558 23 14.5C23 19.1944 19.1944 23 14.5 23C9.80558 23 6 19.1944 6 14.5C6 9.80558 9.80558 6 14.5 6ZM14.5 11C13.9477 11 13.5 11.4477 13.5 12V17C13.5 17.5523 13.9477 18 14.5 18C15.0523 18 15.5 17.5523 15.5 17V12C15.5 11.4477 15.0523 11 14.5 11Z" fill="url(#1752500502775-6427239_circle-coin_existing_1_eckaewepf)"/>
			</clipPath>
			<mask id="1752500502775-6427239_circle-coin_mask_kcbcrf2l5">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M14.5 6C19.1944 6 23 9.80558 23 14.5C23 19.1944 19.1944 23 14.5 23C9.80558 23 6 19.1944 6 14.5C6 9.80558 9.80558 6 14.5 6ZM14.5 11C13.9477 11 13.5 11.4477 13.5 12V17C13.5 17.5523 13.9477 18 14.5 18C15.0523 18 15.5 17.5523 15.5 17V12C15.5 11.4477 15.0523 11 14.5 11Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default CircleCoin;