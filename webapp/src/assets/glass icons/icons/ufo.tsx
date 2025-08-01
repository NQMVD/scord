import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Ufo({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M15.5 8.5C15.5 10.9853 13.4853 13 11 13C8.51472 13 6.5 10.9853 6.5 8.5C6.5 6.01472 8.51472 4 11 4C13.4853 4 15.5 6.01472 15.5 8.5Z" fill="url(#1752500502810-5879917_ufo_existing_2_vf6ibsrfd)" mask="url(#1752500502810-5879917_ufo_mask_n8ii5fwel)" data-glass="origin"/>
		<path clipPath="url(#1752500502810-5879917_ufo_clipPath_n2ibz6v4n)" d="M15.5 8.5C15.5 10.9853 13.4853 13 11 13C8.51472 13 6.5 10.9853 6.5 8.5C6.5 6.01472 8.51472 4 11 4C13.4853 4 15.5 6.01472 15.5 8.5Z" fill="url(#1752500502810-5879917_ufo_existing_2_vf6ibsrfd)" data-glass="clone"/>
		<path d="M22.531 7.53373C23.0313 9.40086 18.6798 12.1891 12.8117 13.7615C6.94355 15.3338 1.78092 15.0949 1.28062 13.2277C0.780326 11.3606 5.13181 8.57236 10.9999 7C16.8681 5.42764 22.0307 5.6666 22.531 7.53373Z" fill="url(#1752500502810-5879917_ufo_existing_3_uk6us457n)" data-glass="blur"/>
		<path d="M21.8065 7.72782C21.7704 7.59296 21.6149 7.35994 21.063 7.13746C20.5258 6.92094 19.7436 6.77291 18.7438 6.72663C16.7533 6.63449 14.0747 6.95256 11.1941 7.72442C8.31344 8.49628 5.8346 9.56016 4.15685 10.6352C3.31421 11.1751 2.71078 11.6945 2.35383 12.1506C1.98707 12.6192 1.96894 12.8987 2.00507 13.0336C2.0412 13.1685 2.19667 13.4015 2.74863 13.624C3.28581 13.8405 4.06806 13.9885 5.06777 14.0348C7.05826 14.1269 9.73695 13.8089 12.6176 13.037L12.8117 13.7615L12.2633 13.9033C6.62516 15.317 1.76527 15.0365 1.28062 13.2277C0.780368 11.3606 5.13184 8.57233 10.9999 6.99998L11.5483 6.8581C17.1864 5.44447 22.0463 5.72498 22.531 7.53371L22.5633 7.71106C22.758 9.57361 18.4968 12.2381 12.8117 13.7615L12.6176 13.037C15.4982 12.2652 17.977 11.2013 19.6548 10.1262C20.4974 9.5863 21.1008 9.06697 21.4578 8.61087C21.8245 8.14221 21.8427 7.86267 21.8065 7.72782Z" fill="url(#1752500502810-5879917_ufo_existing_4_nypslaztr)"/>
		<path d="M11.8313 10.0265C9.71532 10.5935 8.14623 11.5989 8.32663 12.2722L8.87111 20.1381C8.94368 21.1865 9.81536 22 10.8663 22L21.2412 22C22.0169 22 22.4972 21.1552 22.1006 20.4886L15.9892 10.219C15.8088 9.54574 13.9473 9.45957 11.8313 10.0265Z" fill="url(#1752500502810-5879917_ufo_existing_5_9l1e71z01)" data-figma-bg-blur-radius="3"/>
		<path d="M5.07314 1.88824L4.44324 0.301014C4.28398 -0.1003 3.71603 -0.10035 3.55669 0.300936L2.92644 1.88824C2.91951 1.90563 2.90577 1.91938 2.88838 1.9263L1.30091 2.55671C0.899694 2.71604 0.899695 3.28396 1.30092 3.44329L2.88838 4.0737C2.90577 4.08063 2.91951 4.09437 2.92644 4.11176L3.55668 5.69906C3.71602 6.10035 4.28397 6.1003 4.44324 5.69899L5.07314 4.11176C5.08008 4.0943 5.09399 4.08061 5.11147 4.0737L6.69907 3.4433C7.10031 3.28398 7.10031 2.71602 6.69907 2.5567L5.11147 1.9263C5.09399 1.91939 5.08008 1.9057 5.07314 1.88824Z" fill="url(#1752500502810-5879917_ufo_existing_6_ldle12y0e)"/>
		<path d="M20 2.25C20 2.94036 19.4404 3.5 18.75 3.5C18.0596 3.5 17.5 2.94036 17.5 2.25C17.5 1.55964 18.0596 1 18.75 1C19.4404 1 20 1.55964 20 2.25Z" fill="url(#1752500502810-5879917_ufo_existing_7_8qjzkjuhg)" data-figma-bg-blur-radius="3"/>
		<path d="M19.25 2.25C19.25 1.97386 19.0261 1.75 18.75 1.75C18.4739 1.75 18.25 1.97386 18.25 2.25C18.25 2.52614 18.4739 2.75 18.75 2.75V3.5C18.0596 3.5 17.5 2.94036 17.5 2.25C17.5 1.55964 18.0596 1 18.75 1C19.4404 1 20 1.55964 20 2.25C20 2.94036 19.4404 3.5 18.75 3.5V2.75C19.0261 2.75 19.25 2.52614 19.25 2.25Z" fill="url(#1752500502810-5879917_ufo_existing_8_mu2amxwzb)"/>
		<defs>
			<linearGradient id="1752500502810-5879917_ufo_existing_2_vf6ibsrfd" gradientUnits="userSpaceOnUse" x1="11" x2="11" y1="4" y2="13">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_3_uk6us457n" gradientUnits="userSpaceOnUse" x1="11" x2="12.812" y1="7" y2="13.761">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_4_nypslaztr" gradientUnits="userSpaceOnUse" x1="11" x2="12.049" y1="7" y2="10.916">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_5_9l1e71z01" gradientUnits="userSpaceOnUse" x1="12.727" x2="16.469" y1="9.786" y2="23.75">
				<stop stopColor="#151515"/>
				<stop offset="1" stopColor="#575757"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_6_ldle12y0e" gradientUnits="userSpaceOnUse" x1="4" x2="4" y1="0" y2="6">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_7_8qjzkjuhg" gradientUnits="userSpaceOnUse" x1="18.75" x2="18.75" y1="1" y2="3.5">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502810-5879917_ufo_existing_8_mu2amxwzb" gradientUnits="userSpaceOnUse" x1="18.75" x2="18.75" y1="1" y2="2.448">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502810-5879917_ufo_clipPath_n2ibz6v4n">
				<path d="M22.531 7.53373C23.0313 9.40086 18.6798 12.1891 12.8117 13.7615C6.94355 15.3338 1.78092 15.0949 1.28062 13.2277C0.780326 11.3606 5.13181 8.57236 10.9999 7C16.8681 5.42764 22.0307 5.6666 22.531 7.53373Z" fill="url(#1752500502810-5879917_ufo_existing_3_uk6us457n)"/>
			</clipPath>
			<mask id="1752500502810-5879917_ufo_mask_n8ii5fwel">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M22.531 7.53373C23.0313 9.40086 18.6798 12.1891 12.8117 13.7615C6.94355 15.3338 1.78092 15.0949 1.28062 13.2277C0.780326 11.3606 5.13181 8.57236 10.9999 7C16.8681 5.42764 22.0307 5.6666 22.531 7.53373Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Ufo;