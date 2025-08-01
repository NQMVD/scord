import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function CircleArrowLeft({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M16.0001 7.61319C15.9999 5.90661 13.9844 4.90601 12.5177 5.88467L5.94444 10.2714C4.68584 11.1112 4.68584 12.8886 5.94444 13.7284L12.5177 18.1151C13.9845 19.0939 16.0001 18.0934 16.0001 16.3866V13.4999H21.0001L21.1534 13.4921C21.9098 13.4153 22.5001 12.7766 22.5001 11.9999C22.5 11.2233 21.9098 10.5845 21.1534 10.5077L21.0001 10.4999H16.0001V7.61319Z" fill="url(#1752500502773-8525463_circle-arrow-left_existing_0_c468okw2b)" mask="url(#1752500502773-8525463_circle-arrow-left_mask_zmde5dzkd)" data-glass="origin"/>
		<path clipPath="url(#1752500502773-8525463_circle-arrow-left_clipPath_bqundae1k)" d="M16.0001 7.61319C15.9999 5.90661 13.9844 4.90601 12.5177 5.88467L5.94444 10.2714C4.68584 11.1112 4.68584 12.8886 5.94444 13.7284L12.5177 18.1151C13.9845 19.0939 16.0001 18.0934 16.0001 16.3866V13.4999H21.0001L21.1534 13.4921C21.9098 13.4153 22.5001 12.7766 22.5001 11.9999C22.5 11.2233 21.9098 10.5845 21.1534 10.5077L21.0001 10.4999H16.0001V7.61319Z" fill="url(#1752500502773-8525463_circle-arrow-left_existing_0_c468okw2b)" data-glass="clone"/>
		<path d="M23 12C23 5.92487 18.0751 1 12 1C5.92487 1 1 5.92487 1 12C1 18.0751 5.92487 23 12 23C18.0751 23 23 18.0751 23 12ZM15 15.6387C15 17.0545 13.406 17.8842 12.2461 17.0723L7.04785 13.4336C6.05282 12.7369 6.05286 11.2631 7.04785 10.5664L12.2461 6.92773C13.406 6.11583 15 6.94554 15 8.36133V15.6387Z" fill="url(#1752500502773-8525463_circle-arrow-left_existing_1_ysch4osfc)" fillRule="evenodd" data-glass="blur"/>
		<path d="M22.25 12C22.25 6.33908 17.6609 1.75 12 1.75C6.33908 1.75 1.75 6.33908 1.75 12C1.75 17.6609 6.33908 22.25 12 22.25V23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1L12.5664 1.01465C18.3783 1.30945 23 6.11484 23 12L22.9854 12.5664C22.6906 18.3783 17.8852 23 12 23V22.25C17.6609 22.25 22.25 17.6609 22.25 12Z" fill="url(#1752500502773-8525463_circle-arrow-left_existing_2_rnhblp4so)"/>
		<defs>
			<linearGradient id="1752500502773-8525463_circle-arrow-left_existing_0_c468okw2b" gradientUnits="userSpaceOnUse" x1="13.75" x2="13.75" y1="5.5" y2="18.5">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502773-8525463_circle-arrow-left_existing_1_ysch4osfc" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="23">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502773-8525463_circle-arrow-left_existing_2_rnhblp4so" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="13.74">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502773-8525463_circle-arrow-left_clipPath_bqundae1k">
				<path d="M23 12C23 5.92487 18.0751 1 12 1C5.92487 1 1 5.92487 1 12C1 18.0751 5.92487 23 12 23C18.0751 23 23 18.0751 23 12ZM15 15.6387C15 17.0545 13.406 17.8842 12.2461 17.0723L7.04785 13.4336C6.05282 12.7369 6.05286 11.2631 7.04785 10.5664L12.2461 6.92773C13.406 6.11583 15 6.94554 15 8.36133V15.6387Z" fill="url(#1752500502773-8525463_circle-arrow-left_existing_1_ysch4osfc)" fillRule="evenodd"/>
			</clipPath>
			<mask id="1752500502773-8525463_circle-arrow-left_mask_zmde5dzkd">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M23 12C23 5.92487 18.0751 1 12 1C5.92487 1 1 5.92487 1 12C1 18.0751 5.92487 23 12 23C18.0751 23 23 18.0751 23 12ZM15 15.6387C15 17.0545 13.406 17.8842 12.2461 17.0723L7.04785 13.4336C6.05282 12.7369 6.05286 11.2631 7.04785 10.5664L12.2461 6.92773C13.406 6.11583 15 6.94554 15 8.36133V15.6387Z" fill="#000" fillRule="evenodd"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default CircleArrowLeft;