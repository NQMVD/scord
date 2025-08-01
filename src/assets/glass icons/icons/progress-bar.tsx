import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function ProgressBar({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M14 13C15.1046 13 16 13.8954 16 15C16 16.1046 15.1046 17 14 17H6C4.89543 17 4 16.1046 4 15C4 13.8954 4.89543 13 6 13H14Z" fill="url(#1752500502800-2312950_progress-bar_existing_0_oz64ibitq)" mask="url(#1752500502800-2312950_progress-bar_mask_7gtbc6m3j)" data-glass="origin"/>
		<path clipPath="url(#1752500502800-2312950_progress-bar_clipPath_zwgu8hwru)" d="M14 13C15.1046 13 16 13.8954 16 15C16 16.1046 15.1046 17 14 17H6C4.89543 17 4 16.1046 4 15C4 13.8954 4.89543 13 6 13H14Z" fill="url(#1752500502800-2312950_progress-bar_existing_0_oz64ibitq)" data-glass="clone"/>
		<path d="M13.0515 6.36438C12.5595 7.12978 11.4406 7.12978 10.9486 6.36437L8.73817 2.92595C8.20338 2.09406 8.80069 1 9.78964 1L14.2105 1C15.1994 1 15.7967 2.09406 15.262 2.92595L13.0515 6.36438Z" fill="url(#1752500502800-2312950_progress-bar_existing_1_skyjdgqxj)"/>
		<path d="M18 10C20.7614 10 23 12.2386 23 15C23 17.7614 20.7614 20 18 20H6C3.23858 20 1 17.7614 1 15C1 12.2386 3.23858 10 6 10H18ZM6 14C5.44772 14 5 14.4477 5 15C5 15.5523 5.44772 16 6 16H14C14.5523 16 15 15.5523 15 15C15 14.4477 14.5523 14 14 14H6Z" fill="url(#1752500502800-2312950_progress-bar_existing_2_kzpq8iwr7)" data-glass="blur"/>
		<path d="M18 19.25V20H6V19.25H18ZM22.25 15C22.25 12.6528 20.3472 10.75 18 10.75H6C3.65279 10.75 1.75 12.6528 1.75 15C1.75 17.3472 3.65279 19.25 6 19.25V20C3.23858 20 1 17.7614 1 15C1 12.2386 3.23858 10 6 10H18C20.7614 10 23 12.2386 23 15C23 17.7614 20.7614 20 18 20V19.25C20.3472 19.25 22.25 17.3472 22.25 15Z" fill="url(#1752500502800-2312950_progress-bar_existing_3_sxb789dcv)"/>
		<defs>
			<linearGradient id="1752500502800-2312950_progress-bar_existing_0_oz64ibitq" gradientUnits="userSpaceOnUse" x1="10" x2="10" y1="13" y2="17">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502800-2312950_progress-bar_existing_1_skyjdgqxj" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="6.938">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502800-2312950_progress-bar_existing_2_kzpq8iwr7" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="10" y2="20">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502800-2312950_progress-bar_existing_3_sxb789dcv" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="10" y2="15.791">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502800-2312950_progress-bar_clipPath_zwgu8hwru">
				<path d="M18 10C20.7614 10 23 12.2386 23 15C23 17.7614 20.7614 20 18 20H6C3.23858 20 1 17.7614 1 15C1 12.2386 3.23858 10 6 10H18ZM6 14C5.44772 14 5 14.4477 5 15C5 15.5523 5.44772 16 6 16H14C14.5523 16 15 15.5523 15 15C15 14.4477 14.5523 14 14 14H6Z" fill="url(#1752500502800-2312950_progress-bar_existing_2_kzpq8iwr7)"/>
			</clipPath>
			<mask id="1752500502800-2312950_progress-bar_mask_7gtbc6m3j">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M18 10C20.7614 10 23 12.2386 23 15C23 17.7614 20.7614 20 18 20H6C3.23858 20 1 17.7614 1 15C1 12.2386 3.23858 10 6 10H18ZM6 14C5.44772 14 5 14.4477 5 15C5 15.5523 5.44772 16 6 16H14C14.5523 16 15 15.5523 15 15C15 14.4477 14.5523 14 14 14H6Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default ProgressBar;