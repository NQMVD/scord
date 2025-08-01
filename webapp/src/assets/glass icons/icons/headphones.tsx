import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Headphones({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M20 17V11C20 6.58172 16.4183 3 12 3C7.58172 3 4 6.58172 4 11V17C4 17.5523 3.55228 18 3 18C2.44772 18 2 17.5523 2 17V11C2 5.47715 6.47715 1 12 1C17.5228 1 22 5.47715 22 11V17C22 17.5523 21.5523 18 21 18C20.4477 18 20 17.5523 20 17Z" fill="url(#1752500502790-7879774_headphones_existing_0_kt1p4bwe2)" mask="url(#1752500502790-7879774_headphones_mask_z077oiqxy)" data-glass="origin"/>
		<path clipPath="url(#1752500502790-7879774_headphones_clipPath_01tja0kb8)" d="M20 17V11C20 6.58172 16.4183 3 12 3C7.58172 3 4 6.58172 4 11V17C4 17.5523 3.55228 18 3 18C2.44772 18 2 17.5523 2 17V11C2 5.47715 6.47715 1 12 1C17.5228 1 22 5.47715 22 11V17C22 17.5523 21.5523 18 21 18C20.4477 18 20 17.5523 20 17Z" fill="url(#1752500502790-7879774_headphones_existing_0_kt1p4bwe2)" data-glass="clone"/>
		<path d="M6.83301 11C7.47734 11 8 11.5227 8 12.167V20.833C8 21.4773 7.47734 22 6.83301 22H6.5C3.46243 22 1 19.5376 1 16.5C1 13.4624 3.46243 11 6.5 11H6.83301ZM17.5 11C20.5376 11 23 13.4624 23 16.5C23 19.5376 20.5376 22 17.5 22H17.167C16.5227 22 16 21.4773 16 20.833V12.167C16 11.5227 16.5227 11 17.167 11H17.5Z" fill="url(#1752500502790-7879774_headphones_existing_1_c7ar7hlxy)" data-glass="blur"/>
		<path d="M6.83301 21.25V22H6.5V21.25H6.83301ZM7.25 20.833V12.167C7.25 11.9369 7.06313 11.75 6.83301 11.75H6.5C3.87665 11.75 1.75 13.8766 1.75 16.5C1.75 19.1234 3.87665 21.25 6.5 21.25V22C3.46243 22 1 19.5376 1 16.5C1 13.4624 3.46243 11 6.5 11H6.83301C7.47734 11 8 11.5227 8 12.167V20.833C8 21.4773 7.47734 22 6.83301 22V21.25C7.06313 21.25 7.25 21.0631 7.25 20.833Z" fill="url(#1752500502790-7879774_headphones_existing_2_rm6206ej4)"/>
		<path d="M17.167 21.25V22H17.5V21.25H17.167ZM16.75 20.833V12.167C16.75 11.9369 16.9369 11.75 17.167 11.75H17.5C20.1234 11.75 22.25 13.8766 22.25 16.5C22.25 19.1234 20.1234 21.25 17.5 21.25V22C20.5376 22 23 19.5376 23 16.5C23 13.4624 20.5376 11 17.5 11H17.167C16.5227 11 16 11.5227 16 12.167V20.833C16 21.4773 16.5227 22 17.167 22V21.25C16.9369 21.25 16.75 21.0631 16.75 20.833Z" fill="url(#1752500502790-7879774_headphones_existing_3_djol1shh2)"/>
		<defs>
			<linearGradient id="1752500502790-7879774_headphones_existing_0_kt1p4bwe2" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="18">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502790-7879774_headphones_existing_1_c7ar7hlxy" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="11" y2="22">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502790-7879774_headphones_existing_2_rm6206ej4" gradientUnits="userSpaceOnUse" x1="4.5" x2="4.5" y1="11" y2="17.37">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502790-7879774_headphones_existing_3_djol1shh2" gradientUnits="userSpaceOnUse" x1="19.5" x2="19.5" y1="11" y2="17.37">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502790-7879774_headphones_clipPath_01tja0kb8">
				<path d="M6.83301 11C7.47734 11 8 11.5227 8 12.167V20.833C8 21.4773 7.47734 22 6.83301 22H6.5C3.46243 22 1 19.5376 1 16.5C1 13.4624 3.46243 11 6.5 11H6.83301ZM17.5 11C20.5376 11 23 13.4624 23 16.5C23 19.5376 20.5376 22 17.5 22H17.167C16.5227 22 16 21.4773 16 20.833V12.167C16 11.5227 16.5227 11 17.167 11H17.5Z" fill="url(#1752500502790-7879774_headphones_existing_1_c7ar7hlxy)"/>
			</clipPath>
			<mask id="1752500502790-7879774_headphones_mask_z077oiqxy">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M6.83301 11C7.47734 11 8 11.5227 8 12.167V20.833C8 21.4773 7.47734 22 6.83301 22H6.5C3.46243 22 1 19.5376 1 16.5C1 13.4624 3.46243 11 6.5 11H6.83301ZM17.5 11C20.5376 11 23 13.4624 23 16.5C23 19.5376 20.5376 22 17.5 22H17.167C16.5227 22 16 21.4773 16 20.833V12.167C16 11.5227 16.5227 11 17.167 11H17.5Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Headphones;