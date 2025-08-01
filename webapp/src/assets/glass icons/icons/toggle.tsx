import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Toggle({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M18 7C20.7614 7 23 9.23858 23 12C23 14.7614 20.7614 17 18 17H9C6.23858 17 4 14.7614 4 12C4 9.23858 6.23858 7 9 7L18 7Z" fill="url(#1752500502809-8829935_toggle_existing_0_5rfpphjrn)" mask="url(#1752500502809-8829935_toggle_mask_7d4cqrlv8)" data-glass="origin"/>
		<path clipPath="url(#1752500502809-8829935_toggle_clipPath_aj96uqc51)" d="M18 7C20.7614 7 23 9.23858 23 12C23 14.7614 20.7614 17 18 17H9C6.23858 17 4 14.7614 4 12C4 9.23858 6.23858 7 9 7L18 7Z" fill="url(#1752500502809-8829935_toggle_existing_0_5rfpphjrn)" data-glass="clone"/>
		<path d="M15 12C15 15.866 11.866 19 8 19C4.13401 19 1 15.866 1 12C1 8.13401 4.13401 5 8 5C11.866 5 15 8.13401 15 12Z" fill="url(#1752500502809-8829935_toggle_existing_1_flrewlcmo)" data-glass="blur"/>
		<path d="M14.25 12C14.25 8.54822 11.4518 5.75 8 5.75C4.54822 5.75 1.75 8.54822 1.75 12C1.75 15.4518 4.54822 18.25 8 18.25V19C4.13401 19 1 15.866 1 12C1 8.13401 4.13401 5 8 5C11.866 5 15 8.13401 15 12C15 15.866 11.866 19 8 19V18.25C11.4518 18.25 14.25 15.4518 14.25 12Z" fill="url(#1752500502809-8829935_toggle_existing_2_bjt1eu4wf)"/>
		<defs>
			<linearGradient id="1752500502809-8829935_toggle_existing_0_5rfpphjrn" gradientUnits="userSpaceOnUse" x1="13.5" x2="13.5" y1="7" y2="17">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502809-8829935_toggle_existing_1_flrewlcmo" gradientUnits="userSpaceOnUse" x1="8" x2="8" y1="5" y2="19">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502809-8829935_toggle_existing_2_bjt1eu4wf" gradientUnits="userSpaceOnUse" x1="8" x2="8" y1="5" y2="13.107">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502809-8829935_toggle_clipPath_aj96uqc51">
				<path d="M15 12C15 15.866 11.866 19 8 19C4.13401 19 1 15.866 1 12C1 8.13401 4.13401 5 8 5C11.866 5 15 8.13401 15 12Z" fill="url(#1752500502809-8829935_toggle_existing_1_flrewlcmo)"/>
			</clipPath>
			<mask id="1752500502809-8829935_toggle_mask_7d4cqrlv8">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M15 12C15 15.866 11.866 19 8 19C4.13401 19 1 15.866 1 12C1 8.13401 4.13401 5 8 5C11.866 5 15 8.13401 15 12Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Toggle;