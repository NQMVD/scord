import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Connect({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M17.2929 5.29295C17.6834 4.90243 18.3164 4.90243 18.707 5.29295C19.0975 5.68348 19.0975 6.31649 18.707 6.70702L6.70696 18.707C6.31643 19.0975 5.68342 19.0975 5.29289 18.707C4.90237 18.3165 4.90237 17.6835 5.29289 17.293L17.2929 5.29295Z" fill="url(#1752500502780-235646_connect_existing_0_cis0ai36q)" mask="url(#1752500502780-235646_connect_mask_c82leb9n9)" data-glass="origin"/>
		<path clipPath="url(#1752500502780-235646_connect_clipPath_qk9njuv6m)" d="M17.2929 5.29295C17.6834 4.90243 18.3164 4.90243 18.707 5.29295C19.0975 5.68348 19.0975 6.31649 18.707 6.70702L6.70696 18.707C6.31643 19.0975 5.68342 19.0975 5.29289 18.707C4.90237 18.3165 4.90237 17.6835 5.29289 17.293L17.2929 5.29295Z" fill="url(#1752500502780-235646_connect_existing_0_cis0ai36q)" data-glass="clone"/>
		<path d="M10 6C10 8.20914 8.20914 10 6 10C3.79086 10 2 8.20914 2 6C2 3.79086 3.79086 2 6 2C8.20914 2 10 3.79086 10 6Z" fill="url(#1752500502780-235646_connect_existing_1_c8ly6ence)"/>
		<path d="M6 14C8.20914 14 10 15.7909 10 18C10 20.2091 8.20914 22 6 22C3.79086 22 2 20.2091 2 18C2 15.7909 3.79086 14 6 14ZM18 2C20.2091 2 22 3.79086 22 6C22 8.20914 20.2091 10 18 10C15.7909 10 14 8.20914 14 6C14 3.79086 15.7909 2 18 2Z" fill="url(#1752500502780-235646_connect_existing_2_t27k8471q)" data-glass="blur"/>
		<path d="M9.25 18C9.25 16.2051 7.79493 14.75 6 14.75C4.20507 14.75 2.75 16.2051 2.75 18C2.75 19.7949 4.20507 21.25 6 21.25V22C3.79086 22 2 20.2091 2 18C2 15.7909 3.79086 14 6 14C8.20914 14 10 15.7909 10 18C10 20.2091 8.20914 22 6 22V21.25C7.79493 21.25 9.25 19.7949 9.25 18Z" fill="url(#1752500502780-235646_connect_existing_3_x8ukk6dtf)"/>
		<path d="M21.25 6C21.25 4.20507 19.7949 2.75 18 2.75C16.2051 2.75 14.75 4.20507 14.75 6C14.75 7.79493 16.2051 9.25 18 9.25V10C15.7909 10 14 8.20914 14 6C14 3.79086 15.7909 2 18 2C20.2091 2 22 3.79086 22 6C22 8.20914 20.2091 10 18 10V9.25C19.7949 9.25 21.25 7.79493 21.25 6Z" fill="url(#1752500502780-235646_connect_existing_4_5xf7deqkw)"/>
		<path d="M22 18C22 20.2091 20.2091 22 18 22C15.7909 22 14 20.2091 14 18C14 15.7909 15.7909 14 18 14C20.2091 14 22 15.7909 22 18Z" fill="url(#1752500502780-235646_connect_existing_5_9b6gn7s9u)"/>
		<defs>
			<linearGradient id="1752500502780-235646_connect_existing_0_cis0ai36q" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="5" y2="19">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502780-235646_connect_existing_1_c8ly6ence" gradientUnits="userSpaceOnUse" x1="6" x2="6" y1="2" y2="10">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502780-235646_connect_existing_2_t27k8471q" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="2" y2="22">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502780-235646_connect_existing_3_x8ukk6dtf" gradientUnits="userSpaceOnUse" x1="6" x2="6" y1="14" y2="18.633">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502780-235646_connect_existing_4_5xf7deqkw" gradientUnits="userSpaceOnUse" x1="18" x2="18" y1="2" y2="6.633">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502780-235646_connect_existing_5_9b6gn7s9u" gradientUnits="userSpaceOnUse" x1="18" x2="18" y1="14" y2="22">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<clipPath id="1752500502780-235646_connect_clipPath_qk9njuv6m">
				<path d="M6 14C8.20914 14 10 15.7909 10 18C10 20.2091 8.20914 22 6 22C3.79086 22 2 20.2091 2 18C2 15.7909 3.79086 14 6 14ZM18 2C20.2091 2 22 3.79086 22 6C22 8.20914 20.2091 10 18 10C15.7909 10 14 8.20914 14 6C14 3.79086 15.7909 2 18 2Z" fill="url(#1752500502780-235646_connect_existing_2_t27k8471q)"/>
			</clipPath>
			<mask id="1752500502780-235646_connect_mask_c82leb9n9">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M6 14C8.20914 14 10 15.7909 10 18C10 20.2091 8.20914 22 6 22C3.79086 22 2 20.2091 2 18C2 15.7909 3.79086 14 6 14ZM18 2C20.2091 2 22 3.79086 22 6C22 8.20914 20.2091 10 18 10C15.7909 10 14 8.20914 14 6C14 3.79086 15.7909 2 18 2Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Connect;