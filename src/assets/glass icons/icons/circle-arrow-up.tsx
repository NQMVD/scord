import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function CircleArrowUp({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M7.61319 16C5.90661 15.9998 4.90601 13.9843 5.88467 12.5176L10.2714 5.94432C11.1112 4.68571 12.8886 4.68571 13.7284 5.94432L18.1151 12.5176C19.0939 13.9843 18.0934 16 16.3866 16H13.4999V21L13.4921 21.1533C13.4153 21.9097 12.7766 22.5 11.9999 22.5C11.2233 22.4999 10.5845 21.9096 10.5077 21.1533L10.4999 21V16H7.61319Z" fill="url(#1752500502774-770253_circle-arrow-up_existing_0_hj876f6rt)" mask="url(#1752500502774-770253_circle-arrow-up_mask_flossmv2j)" data-glass="origin"/>
		<path clipPath="url(#1752500502774-770253_circle-arrow-up_clipPath_jfo4obdkz)" d="M7.61319 16C5.90661 15.9998 4.90601 13.9843 5.88467 12.5176L10.2714 5.94432C11.1112 4.68571 12.8886 4.68571 13.7284 5.94432L18.1151 12.5176C19.0939 13.9843 18.0934 16 16.3866 16H13.4999V21L13.4921 21.1533C13.4153 21.9097 12.7766 22.5 11.9999 22.5C11.2233 22.4999 10.5845 21.9096 10.5077 21.1533L10.4999 21V16H7.61319Z" fill="url(#1752500502774-770253_circle-arrow-up_existing_0_hj876f6rt)" data-glass="clone"/>
		<path d="M12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23ZM15.6387 15C17.0545 15 17.8842 13.406 17.0723 12.2461L13.4336 7.04785C12.7369 6.05282 11.2631 6.05286 10.5664 7.04785L6.92773 12.2461C6.11583 13.406 6.94554 15 8.36133 15H15.6387Z" fill="url(#1752500502774-770253_circle-arrow-up_existing_1_12cyo61yv)" fillRule="evenodd" data-glass="blur"/>
		<path d="M22.25 12C22.25 6.33908 17.6609 1.75 12 1.75C6.33908 1.75 1.75 6.33908 1.75 12C1.75 17.6609 6.33908 22.25 12 22.25V23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1L12.5664 1.01465C18.3783 1.30945 23 6.11484 23 12L22.9854 12.5664C22.6906 18.3783 17.8852 23 12 23V22.25C17.6609 22.25 22.25 17.6609 22.25 12Z" fill="url(#1752500502774-770253_circle-arrow-up_existing_2_3kl4u3pug)"/>
		<defs>
			<linearGradient id="1752500502774-770253_circle-arrow-up_existing_0_hj876f6rt" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="5" y2="22.5">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502774-770253_circle-arrow-up_existing_1_12cyo61yv" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="23">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502774-770253_circle-arrow-up_existing_2_3kl4u3pug" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="13.74">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502774-770253_circle-arrow-up_clipPath_jfo4obdkz">
				<path d="M12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23ZM15.6387 15C17.0545 15 17.8842 13.406 17.0723 12.2461L13.4336 7.04785C12.7369 6.05282 11.2631 6.05286 10.5664 7.04785L6.92773 12.2461C6.11583 13.406 6.94554 15 8.36133 15H15.6387Z" fill="url(#1752500502774-770253_circle-arrow-up_existing_1_12cyo61yv)" fillRule="evenodd"/>
			</clipPath>
			<mask id="1752500502774-770253_circle-arrow-up_mask_flossmv2j">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23ZM15.6387 15C17.0545 15 17.8842 13.406 17.0723 12.2461L13.4336 7.04785C12.7369 6.05282 11.2631 6.05286 10.5664 7.04785L6.92773 12.2461C6.11583 13.406 6.94554 15 8.36133 15H15.6387Z" fill="#000" fillRule="evenodd"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default CircleArrowUp;