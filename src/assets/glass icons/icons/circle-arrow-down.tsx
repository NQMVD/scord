import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function CircleArrowDown({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M16.3867 8C18.0933 8.0002 19.0939 10.0157 18.1152 11.4824L13.7285 18.0557C12.8886 19.3143 11.1113 19.3143 10.2715 18.0557L5.88474 11.4824C4.90598 10.0157 5.90651 8 7.61326 8L10.5 8L10.5 3L10.5078 2.84668C10.5846 2.09029 11.2233 1.5 12 1.5C12.7765 1.50011 13.4154 2.09035 13.4922 2.84668L13.5 3V8L16.3867 8Z" fill="url(#1752500502772-8465469_circle-arrow-down_existing_0_k1hyq6zj1)" mask="url(#1752500502772-8465469_circle-arrow-down_mask_2r1aebsle)" data-glass="origin"/>
		<path clipPath="url(#1752500502772-8465469_circle-arrow-down_clipPath_tgvjyr4st)" d="M16.3867 8C18.0933 8.0002 19.0939 10.0157 18.1152 11.4824L13.7285 18.0557C12.8886 19.3143 11.1113 19.3143 10.2715 18.0557L5.88474 11.4824C4.90598 10.0157 5.90651 8 7.61326 8L10.5 8L10.5 3L10.5078 2.84668C10.5846 2.09029 11.2233 1.5 12 1.5C12.7765 1.50011 13.4154 2.09035 13.4922 2.84668L13.5 3V8L16.3867 8Z" fill="url(#1752500502772-8465469_circle-arrow-down_existing_0_k1hyq6zj1)" data-glass="clone"/>
		<path d="M12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1ZM8.36133 9C6.94554 9 6.11583 10.594 6.92773 11.7539L10.5664 16.9521C11.2631 17.9472 12.7369 17.9471 13.4336 16.9521L17.0723 11.7539C17.8842 10.594 17.0545 9 15.6387 9L8.36133 9Z" fill="url(#1752500502772-8465469_circle-arrow-down_existing_1_ux3v72uwl)" fillRule="evenodd" data-glass="blur"/>
		<path d="M22.25 12C22.25 6.33908 17.6609 1.75 12 1.75C6.33908 1.75 1.75 6.33908 1.75 12C1.75 17.6609 6.33908 22.25 12 22.25V23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1L12.5664 1.01465C18.3783 1.30945 23 6.11484 23 12L22.9854 12.5664C22.6906 18.3783 17.8852 23 12 23V22.25C17.6609 22.25 22.25 17.6609 22.25 12Z" fill="url(#1752500502772-8465469_circle-arrow-down_existing_2_hkzch8ukj)"/>
		<defs>
			<linearGradient id="1752500502772-8465469_circle-arrow-down_existing_0_k1hyq6zj1" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1.5" y2="19">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502772-8465469_circle-arrow-down_existing_1_ux3v72uwl" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="23">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502772-8465469_circle-arrow-down_existing_2_hkzch8ukj" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="13.74">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502772-8465469_circle-arrow-down_clipPath_tgvjyr4st">
				<path d="M12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1ZM8.36133 9C6.94554 9 6.11583 10.594 6.92773 11.7539L10.5664 16.9521C11.2631 17.9472 12.7369 17.9471 13.4336 16.9521L17.0723 11.7539C17.8842 10.594 17.0545 9 15.6387 9L8.36133 9Z" fill="url(#1752500502772-8465469_circle-arrow-down_existing_1_ux3v72uwl)" fillRule="evenodd"/>
			</clipPath>
			<mask id="1752500502772-8465469_circle-arrow-down_mask_2r1aebsle">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M12 1C18.0751 1 23 5.92487 23 12C23 18.0751 18.0751 23 12 23C5.92487 23 1 18.0751 1 12C1 5.92487 5.92487 1 12 1ZM8.36133 9C6.94554 9 6.11583 10.594 6.92773 11.7539L10.5664 16.9521C11.2631 17.9472 12.7369 17.9471 13.4336 16.9521L17.0723 11.7539C17.8842 10.594 17.0545 9 15.6387 9L8.36133 9Z" fill="#000" fillRule="evenodd"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default CircleArrowDown;