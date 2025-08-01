import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Swap({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M8.5 1.5C12.366 1.5 15.5 4.63401 15.5 8.5C15.5 12.366 12.366 15.5 8.5 15.5C4.63401 15.5 1.5 12.366 1.5 8.5C1.5 4.63401 4.63401 1.5 8.5 1.5Z" fill="url(#1753173464874-1572285_swap_existing_0_1otkkfops)" fillRule="evenodd" mask="url(#1753173464874-1572285_swap_mask_wtclzdgrt)" data-glass="origin"/>
		<path clipPath="url(#1753173464874-1572285_swap_clipPath_wxh4er2t8)" d="M8.5 1.5C12.366 1.5 15.5 4.63401 15.5 8.5C15.5 12.366 12.366 15.5 8.5 15.5C4.63401 15.5 1.5 12.366 1.5 8.5C1.5 4.63401 4.63401 1.5 8.5 1.5Z" fill="url(#1753173464874-1572285_swap_existing_0_1otkkfops)" fillRule="evenodd" data-glass="clone"/>
		<path d="M19.5574 2.44926C19.9526 1.85025 20.8315 1.85025 21.2267 2.44926L22.8329 4.88353C23.2716 5.54838 22.7948 6.43427 21.9983 6.43427H18.7858C17.9893 6.43427 17.5125 5.54838 17.9512 4.88353L19.5574 2.44926Z" fill="url(#1753173464874-1572285_swap_existing_1_o4tz5hmmm)"/>
		<path d="M15.5 8.5C19.366 8.5 22.5 11.634 22.5 15.5C22.5 19.366 19.366 22.5 15.5 22.5C11.634 22.5 8.5 19.366 8.5 15.5C8.5 11.634 11.634 8.5 15.5 8.5Z" fill="url(#1753173464874-1572285_swap_existing_2_j46yz6w0g)" fillRule="evenodd" data-glass="blur"/>
		<path d="M15.5 8.5C19.366 8.5 22.5 11.634 22.5 15.5C22.5 19.366 19.366 22.5 15.5 22.5C11.634 22.5 8.5 19.366 8.5 15.5C8.5 11.634 11.634 8.5 15.5 8.5ZM15.5 9.25C12.0482 9.25 9.25 12.0482 9.25 15.5C9.25 18.9518 12.0482 21.75 15.5 21.75C18.9518 21.75 21.75 18.9518 21.75 15.5C21.75 12.0482 18.9518 9.25 15.5 9.25Z" fill="url(#1753173464874-1572285_swap_existing_3_n6lbgc7yp)"/>
		<path d="M5.44255 21.5507C5.0473 22.1498 4.16844 22.1498 3.7732 21.5507L2.16699 19.1165C1.7283 18.4516 2.20512 17.5657 3.00167 17.5657H6.21407C7.01062 17.5657 7.48744 18.4516 7.04875 19.1165L5.44255 21.5507Z" fill="url(#1753173464874-1572285_swap_existing_4_w9c1wsn27)"/>
		<defs>
			<linearGradient id="1753173464874-1572285_swap_existing_0_1otkkfops" gradientUnits="userSpaceOnUse" x1="8.5" x2="8.5" y1="1.5" y2="15.5">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1753173464874-1572285_swap_existing_1_o4tz5hmmm" gradientUnits="userSpaceOnUse" x1="20.392" x2="20.392" y1="2" y2="6.434">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1753173464874-1572285_swap_existing_2_j46yz6w0g" gradientUnits="userSpaceOnUse" x1="15.5" x2="15.5" y1="8.5" y2="22.5">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1753173464874-1572285_swap_existing_3_n6lbgc7yp" gradientUnits="userSpaceOnUse" x1="15.5" x2="15.5" y1="8.5" y2="16.608">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1753173464874-1572285_swap_existing_4_w9c1wsn27" gradientUnits="userSpaceOnUse" x1="4.608" x2="4.608" y1="17.566" y2="22">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<clipPath id="1753173464874-1572285_swap_clipPath_wxh4er2t8">
				<path d="M15.5 8.5C19.366 8.5 22.5 11.634 22.5 15.5C22.5 19.366 19.366 22.5 15.5 22.5C11.634 22.5 8.5 19.366 8.5 15.5C8.5 11.634 11.634 8.5 15.5 8.5Z" fill="url(#1753173464874-1572285_swap_existing_2_j46yz6w0g)" fillRule="evenodd"/>
			</clipPath>
			<mask id="1753173464874-1572285_swap_mask_wtclzdgrt">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M15.5 8.5C19.366 8.5 22.5 11.634 22.5 15.5C22.5 19.366 19.366 22.5 15.5 22.5C11.634 22.5 8.5 19.366 8.5 15.5C8.5 11.634 11.634 8.5 15.5 8.5Z" fill="#000" fillRule="evenodd"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Swap;